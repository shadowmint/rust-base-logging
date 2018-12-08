extern crate time;

use crate::core::traits::Log;
use crate::core::traits::Loggable;
use crate::core::traits::LogFormatter;
use crate::core::level::Level;
use crate::formatters::DefaultFormatter;

pub struct Logger {
    loggers: Vec<Box<Log + Send>>,
    format: Box<LogFormatter + Send>,
}

impl<'a> Logger {
    pub fn new() -> Logger {
        return Logger {
            loggers: Vec::new(),
            format: Box::new(DefaultFormatter::new()),
        };
    }

    pub fn with<T>(mut self, logger: T) -> Logger where T: Log + Send + 'static {
        self.loggers.push(Box::new(logger) as Box<Log + Send>);
        return self;
    }

    pub fn with_format<T>(mut self, formatter: T) -> Logger where T: LogFormatter + Send + 'static {
        self.format = Box::new(formatter);
        return self;
    }

    pub fn log<'b, 'c, T>(&'c mut self, level: Level, message: T) where T: Loggable + 'b {
        let now = time::now();
        let raw = self.format.log_format(level, now, message.log_message(), message.log_properties());
        for target in self.loggers.iter_mut() {
            target.log(&raw);
        }
    }
}
