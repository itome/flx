use color_eyre::Result;
use daemon::flutter::FlutterDaemon;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(
        short,
        long,
        value_name = "String",
        help = "Path to the Flutter project"
    )]
    pub project_root: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let flutter_daemon = FlutterDaemon::new()?;
    let results = flutter_daemon
        .get_supported_platforms(args.project_root)
        .await
        .unwrap();
    println!("{:?}", results);

    return Ok(());
}
