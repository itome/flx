use std::sync::Arc;

use color_eyre::{eyre::eyre, Result};
use futures::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::io::{
    request::{EmptyParams, VmServiceRequest},
    response::{GetVersionResponse, GetVersionResult, VmServiceResponse},
};

pub struct VmService {
    incoming_tx: broadcast::Sender<String>,
    outgoing_tx: mpsc::Sender<String>,
    request_count: Arc<Mutex<u32>>,
}

impl VmService {
    pub fn new(uri: String) -> Result<Self> {
        let (incoming_tx, _) = broadcast::channel::<String>(16);
        let (outgoing_tx, mut outgoing_rx) = mpsc::channel::<String>(16);

        let _incoming_tx = incoming_tx.clone();
        tokio::spawn(async move {
            let Ok((stream, _)) = connect_async(uri).await else {
                return;
            };
            let (mut write, mut read) = stream.split();
            loop {
                tokio::select! {
                    Some(Ok(Message::Text(next))) = read.next() => {
                        _incoming_tx.send(next).unwrap();
                    },
                    Some(text) = outgoing_rx.recv() => {
                        write.send(Message::Text(text)).await.unwrap();
                    },
                }
            }
        });

        Ok(Self {
            incoming_tx,
            outgoing_tx,
            request_count: Arc::new(Mutex::new(0)),
        })
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
