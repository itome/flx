use color_eyre::Result;
use devtools::{
    protocols::vm_service::{StreamId, VmServiceProtocol},
    vm_service::VmService,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(
        short,
        long,
        value_name = "String",
        help = "Your Flutter Project's VM Service Connection"
    )]
    pub ws_uri: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let vm_service = VmService::new();
    vm_service.connect(args.ws_uri).await;

    vm_service.stream_listen(StreamId::Extension).await?;

    while let Ok(event) = vm_service.next_event(StreamId::Extension).await {
        println!("{:?}", event);
    }

    return Ok(());
}
