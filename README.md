# logger-rs
A logging utilities library for Rust

## Example
```rust
let style = Arrow::default().colored(true);
let logger = Logger::default().style(Box::new(style));
logger.debug("Hello, world!");
logger.success("Hello, world!");
logger.warn("Hello, world!");
logger.fail("Hello, world!");
```

![demo](https://i.imgur.com/Ex7HAl8.png)
