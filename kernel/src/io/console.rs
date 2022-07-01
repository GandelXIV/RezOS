use super::serial;

const SERIAL_DEBUG_COM: usize = 1;

pub fn init() {}

pub trait Console {
    fn putb(&mut self, bin: &[u8]);
    fn puts(&mut self, s: &str);
}

pub struct SerialConsole {}

impl Console for SerialConsole {
    fn puts(&mut self, s: &str) {
        for c in s.bytes() {
            serial::access(SERIAL_DEBUG_COM).wb(c);
        }
    }
    fn putb(&mut self, bin: &[u8]) {
        for b in bin {
            serial::access(SERIAL_DEBUG_COM).wb(*b);
        }
    }
}
