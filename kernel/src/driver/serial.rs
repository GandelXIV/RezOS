use crate::arch::portio;

const COM1: u16 = 0x3F8;

pub fn write(text: &str) {
    for char in text.bytes() {
        write_com1(char);
    }
}

fn write_com1(byte: u8) {
    unsafe {
        portio::output_byte(COM1, byte);
    }
}

pub fn init() {
    unsafe {
        portio::output_byte(COM1 + 1, 0x00);
    }
    write("Serial initialized!\n");
}
