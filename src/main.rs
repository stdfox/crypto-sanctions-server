mod logger;
mod server;
mod updater;

use std::net::IpAddr;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use log::LevelFilter;
use tokio::sync::RwLock;

const DEFAULT_DATABASE_CAPACITY: usize = 1000;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Database = Arc<RwLock<Vec<String>>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, default_value = "debug", help = "Set output log level")]
    log: LevelFilter,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Check crypto wallet address")]
    Check {
        #[arg(help = "The crypto wallet address to check")]
        address: String,
    },
    #[command(about = "Start http server with json api")]
    Serve {
        #[arg(
            long,
            default_value = "127.0.0.1",
            help = "Set host that the server will use"
        )]
        host: IpAddr,
        #[arg(
            long,
            default_value = "8000",
            help = "Set port that the server will use"
        )]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    logger::try_init(args.log)?;

    let db = Arc::new(RwLock::new(Vec::with_capacity(DEFAULT_DATABASE_CAPACITY)));
    let _ = tokio::spawn(updater::try_update(db.clone())).await??;

    match args.command {
        Commands::Check { address } => {
            let s = db.read().await.contains(&address);
            println!("{{\"address\": \"{}\", \"sanctioned\": {}}}", address, s);
        }
        Commands::Serve { host, port } => tokio::spawn(server::serve(host, port, db)).await??,
    };

    logger::flush();

    Ok(())
}
