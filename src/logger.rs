use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

use crate::Error;

pub(crate) fn try_init(level: LevelFilter) -> Result<(), Error> {
    let target = option_env!("CARGO_CRATE_NAME").unwrap_or_default();

    let colors: ColoredLevelConfig = ColoredLevelConfig::default()
        .error(Color::BrightRed)
        .warn(Color::Yellow)
        .info(Color::BrightWhite)
        .debug(Color::BrightBlack)
        .trace(Color::Magenta);

    Dispatch::new()
        .format(move |out, message, record| {
            let date = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT);
            let thread_id = std::thread::current().id();

            out.finish(format_args!(
                "{}{} {:5} [{}@{}] {}\x1B[0m",
                format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                date.unwrap_or_default(),
                record.level(),
                record.target().replacen(target, "main", 1),
                // see more at https://github.com/rust-lang/rust/issues/67939
                format!("{:?}", thread_id).replace("ThreadId", "thread"),
                message,
            ))
        })
        .level(LevelFilter::Off)
        .level_for(target, level)
        .chain(std::io::stdout())
        .apply()
        .map_err(|e| e.into())
}

pub(crate) fn flush() {
    log::logger().flush();
}
