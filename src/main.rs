use clap::Parser;
use log::LevelFilter;

mod logger;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = false)]
struct Args {
    #[arg(long, default_value = "debug", help = "Output log level")]
    log: LevelFilter,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    logger::try_init(args.log)?;
    logger::flush();

    Ok(())
}
