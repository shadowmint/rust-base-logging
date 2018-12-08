#[allow(dead_code)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod core;

mod infrastructure;

pub mod formatters;
pub mod loggables;
pub mod loggers;

pub use crate::core::traits::Log;
pub use crate::core::traits::Loggable;
pub use crate::core::traits::LogFormatter;
pub use crate::core::logger::Logger;
pub use crate::core::level::Level;
pub use crate::core::logger_ref::LoggerRef;
pub use crate::loggables::Record;

#[cfg(test)]
mod tests {
    use crate::Log;
    use crate::Logger;
    use crate::Level;

    pub struct CustomLogger {}

    impl Log for CustomLogger {
        fn log(&mut self, message: &str) {
            println!("CustomLogger: {}", message);
        }
    }

    #[test]
    fn test_custom_logger() {
        let mut logger = Logger::new().with(CustomLogger {});
        logger.log(Level::Info, format_log!("Hello {}", "Word"));
        logger.log(Level::Info, format_log!({"Hello {}", "Word"}, {"extra" => "property"}));
    }
}
