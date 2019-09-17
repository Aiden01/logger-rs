use logger_rs::style::DefaultStyle;
use logger_rs::{Console, Logger};

fn main() {
    let style = DefaultStyle::default().date(true);
    let logger = Logger::default().style(Box::new(style));
    logger.debug("Hello, world");
}
