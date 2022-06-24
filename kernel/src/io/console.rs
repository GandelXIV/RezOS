use super::serial;

pub fn init() {
    
}

pub trait Console {
    fn puts(&mut self, s: &str);
}

pub struct SerialConsole {}

impl Console for SerialConsole {
    fn puts(&mut self, s: &str) {
        for b in s.bytes() {
            serial::access(1).wb(b);
        }
    }
}