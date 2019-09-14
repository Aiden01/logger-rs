/// A logging utility library for Rust
/// ## Example:
/// ```rust
/// let logger = Logger::default();
/// logger.warn("this is a warning!");
/// // or using a custom style and bridge
/// let logger = Logger::new()
///                  .style(Box::new(Arrow::default()))
///                  .bridge(Box::new(Console::default()))
/// ```
pub mod logger;
pub mod style;
pub use logger::*;
