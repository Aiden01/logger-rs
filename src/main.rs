use logger_rs::{style::Arrow, Logger};
fn main() {
    let style = Arrow::default().colored(true);
    let logger = Logger::default().style(Box::new(style));
    logger.debug("Hello, world!");
    logger.success("Hello, world!");
    logger.warn("Hello, world!");
    logger.fail("Hello, world!");
}
