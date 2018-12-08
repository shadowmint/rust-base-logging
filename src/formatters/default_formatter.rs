extern crate time;

use std::collections::HashMap;
use crate::{Level, LogFormatter};
use self::time::Tm;

pub struct DefaultFormatter {}

impl DefaultFormatter {
    pub fn new() -> DefaultFormatter {
        return DefaultFormatter {};
    }

    fn combine(&self, message: Option<&str>, properties: Option<HashMap<&str, &str>>) -> String {
        let mut rtn = String::new();
        match message {
            Some(msg) => {
                rtn.push_str(msg);
            }
            None => {}
        };
        match properties {
            Some(props) => {
                for (key, value) in props.iter() {
                    rtn.push_str(&format!(" {}:{}", key, value));
                }
            }
            None => {}
        }
        return rtn;
    }
}

impl LogFormatter for DefaultFormatter {
    fn log_format(&self, level: Level, timestamp: Tm, message: Option<&str>, properties: Option<HashMap<&str, &str>>) -> String {
        let timestring = match time::strftime("%b %d %H:%M:%S", &timestamp) {
            Ok(i) => i,
            Err(_) => String::from("")
        };
        return format!("{} {:?} - {}", timestring, level, self.combine(message, properties));
    }
}