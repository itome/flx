use std::sync::Arc;

use color_eyre::{eyre::eyre, Result};
use futures::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::io::{
    event::VmServiceEvent,
    request::{EmptyParams, StreamId, VmServiceRequest},
    response::{GetVersionResponse, GetVersionResult, VmServiceResponse},
    types::{self, Event},
};

pub struct VmService {
    incoming_tx: broadcast::Sender<String>,
    outgoing_tx: mpsc::Sender<String>,
    outgoing_rx: Arc<Mutex<mpsc::Receiver<String>>>,
    request_count: Arc<Mutex<u32>>,
}

impl VmService {
    pub fn new() -> Self {
        let (incoming_tx, _) = broadcast::channel::<String>(16);
        let (outgoing_tx, outgoing_rx) = mpsc::channel::<String>(16);

        Self {
            incoming_tx,
            outgoing_tx,
            outgoing_rx: Arc::new(Mutex::new(outgoing_rx)),
            request_count: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn start_websocket(&self, uri: String) {
        let _incoming_tx = self.incoming_tx.clone();
        let _outgoing_rx = self.outgoing_rx.clone();
        let Ok((stream, _)) = connect_async(uri).await else {
            return;
        };
        let (mut write, mut read) = stream.split();
        tokio::spawn(async move {
            let mut _outgoing_rx = _outgoing_rx.lock().await;
            loop {
                tokio::select! {
                    Some(Ok(Message::Text(next))) = read.next() => {
                        _incoming_tx.send(next).unwrap();
                    },
                    Some(text) = _outgoing_rx.recv() => {
                        write.send(Message::Text(text)).await.unwrap();
                    },
                }
            }
        });
    }

    pub async fn get_version(&self) -> Result<GetVersionResult> {
        let request_id = self.request_id().await;
        let request = VmServiceRequest::GetVersion {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            params: EmptyParams {},
        };
        self.send_request(&request).await?;
        let result: GetVersionResponse = self.receive_response(request_id).await?;
        result.result.ok_or(eyre!("Could not get daemon version"))
    }

    pub async fn stream_listen(&self, stream_id: StreamId) -> Result<()> {
        let request_id = self.request_id().await;
        let request = VmServiceRequest::StreamListen {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            params: super::io::request::StreamListenParams { stream_id },
        };
        self.send_request(&request).await?;
        Ok(())
    }

    pub async fn stream_cancel(&self, stream_id: StreamId) -> Result<()> {
        let request_id = self.request_id().await;
        let request = VmServiceRequest::StreamCancel {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            params: super::io::request::StreamCancelParams { stream_id },
        };
        self.send_request(&request).await?;
        Ok(())
    }

    pub async fn receive_event(&self) -> Result<types::Event> {
        let mut rx = self.incoming_tx.subscribe();
        while let Ok(line) = rx.recv().await {
            let response = serde_json::from_str::<VmServiceEvent>(&line);
            if let Ok(res) = response {
                if res.method == "streamNotify" {
                    return Ok(res.params);
                }
            }
        }
        Err(eyre!("Could not receive daemon response"))
    }

    async fn request_id(&self) -> u32 {
        let mut request_count = self.request_count.lock().await;
        *request_count += 1;
        *request_count
    }

    async fn send_request(&self, request: &VmServiceRequest) -> Result<()> {
        let message = serde_json::to_string(request)?;
        self.outgoing_tx.send(message).await?;
        Ok(())
    }

    async fn receive_response<T>(&self, request_id: u32) -> Result<VmServiceResponse<T>>
    where
        T: DeserializeOwned,
    {
        let mut rx = self.incoming_tx.subscribe();
        while let Ok(line) = rx.recv().await {
            let response = serde_json::from_str::<VmServiceResponse<T>>(&line);
            if let Ok(res) = response {
                if res.id == request_id {
                    return Ok(res);
                }
            }
        }
        Err(eyre!("Could not receive daemon response"))
    }
}
