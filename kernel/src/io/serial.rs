use crate::bootboot::BOOTBOOT_HEADER;
use crate::arch;
use core::marker::Copy;
use core::mem;
use lazy_static::lazy_static;
use spin::Mutex;

pub const COMMON_COM1: u16 = 0x3F8;

lazy_static! {
    static ref SERIAL_PORTS: Mutex<[Option<SerialHandle>; 8]> = Mutex::new([None; 8]);
}

pub fn init(bootboot: &BOOTBOOT_HEADER) -> Result<(), SerialError> {
    // BOOTBOOT creates a serial debug console on COM 1
    // We assume it lives on COMMON_COM1
    unsafe { inherit(1, COMMON_COM1)? }
    // TODO init rest of the coms
    Ok(())
}

#[derive(Debug)]
pub enum SerialError {
    InvalidComId,
    UnavailableHandle,
}

// used to acquire an already existing serial connection,
// for instance a serial debug console initialized by the bootloader
// WARNING: this will overwrite any existing handle
pub unsafe fn inherit(comid: usize, ioport: u16) -> Result<(), SerialError> {
    if comid > 0 && comid < 9 {
        SERIAL_PORTS.lock()[comid - 1] = Some(SerialHandle { ioport });
        return Ok(());
    }
    Err(SerialError::InvalidComId)
}

pub fn access(comid: usize) -> Result<SerialHandle, SerialError> {
    return match SERIAL_PORTS.lock().get(comid - 1) {
        Some(mut handle) => {
            if !handle.is_some() {
                return Err(SerialError::UnavailableHandle);
            }
            return Ok(mem::replace(&mut handle, &None).unwrap());
        }
        None => Err(SerialError::InvalidComId),
    };
}

#[derive(Copy, Clone)]
pub struct SerialHandle {
    ioport: u16,
}

impl SerialHandle {
    pub fn write_byte(&self, b: u8) {
        unsafe {
            arch::portio::output_byte(self.ioport, b);
        }
    }

    pub fn write_char(&self, c: char) {
        unsafe {
            arch::portio::output_long(self.ioport, c.into())
        }
    }

    pub fn write_str(&self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b);
        }
    }
}
