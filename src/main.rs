mod client;
mod logger;

use clap::{Parser, Subcommand};
use log::LevelFilter;

type Error = Box<dyn std::error::Error + Send + Sync>;

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
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    logger::try_init(args.log)?;

    match args.command {
        Commands::Check { address } => {
            let s = client::check_address(address.clone()).await?;
            println!("{{\"address\": \"{}\", \"sanctioned\": {}}}", address, s);
        }
    };

    logger::flush();

    Ok(())
}
