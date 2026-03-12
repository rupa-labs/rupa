/// Standard log levels for the framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// A Port for structured logging.
pub trait Logger: Send + Sync {
    fn log(&self, level: Level, target: &str, message: &str);
    
    fn info(&self, target: &str, msg: &str) { self.log(Level::Info, target, msg); }
    fn warn(&self, target: &str, msg: &str) { self.log(Level::Warn, target, msg); }
    fn error(&self, target: &str, msg: &str) { self.log(Level::Error, target, msg); }
}

/// A default logger that writes to the standard output.
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: Level, target: &str, message: &str) {
        let color = match level {
            Level::Error => "\x1B[31m",
            Level::Warn => "\x1B[33m",
            Level::Info => "\x1B[32m",
            Level::Debug => "\x1B[36m",
            Level::Trace => "\x1B[90m",
        };
        println!("{}[{:?}]\x1B[0m [{}] {}", color, level, target, message);
    }
}

/// A logger that does nothing.
pub struct NullLogger;
impl Logger for NullLogger {
    fn log(&self, _level: Level, _target: &str, _message: &str) {}
}
