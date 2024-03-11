mod database;
mod logger;
mod server;
mod updater;

use std::net::IpAddr;
use std::sync::Arc;

use clap::Parser;
use log::LevelFilter;
use tokio::sync::RwLock;

use crate::database::InMemoryDatabase;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = false)]
struct Args {
    #[arg(long, default_value = "127.0.0.1", help = "Host the server will use")]
    host: IpAddr,
    #[arg(long, default_value = "8000", help = "Port the server will use")]
    port: u16,
    #[arg(long, default_value = "debug", help = "Output log level")]
    log: LevelFilter,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    logger::try_init(args.log)?;

    let db = Arc::new(RwLock::new(InMemoryDatabase::default()));

    let _ = tokio::spawn(updater::try_update(db.clone())).await??;
    let _ = tokio::spawn(server::serve(args.host, args.port, db)).await??;

    logger::flush();

    Ok(())
}
