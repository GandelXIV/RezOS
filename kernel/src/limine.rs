// This module handles all things limine
// See more about the protocol: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md
use core::ffi::CStr;
use core::fmt::Write;
use core::ptr::NonNull;
use core::str;
use lazy_static::lazy_static;
use spin::Mutex;

// first two items in .id of all requests must be equal to the following magic
// TODO: check this in init() for all requests
const MAGIC_COMMON: (u64, u64) = (0xc7b1dd30df4c8b88, 0x0a82e883a194f07b);

// simple pointer wrappers that can be replaced in the future for something like NonNull<T>
type Ptr<T> = *const T;
type MutPtr<T> = *mut T;

// See more: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#terminal-callback
type TerminalCallbackFunction = extern "C" fn(Ptr<Terminal>, u64, u64, u64, u64);

// function provided by limine to simplify display access
// WARNING: The function is NOT thread-safe, NOT reentrant, per-terminal.
// we access it from a Mutex<TerminalWriter> e.g. TERM0
type TerminalWriteFunction = extern "C" fn(Ptr<Terminal>, *const [u8], usize);

// Linked from kentry/limine.asm
extern "C" {
    static LIMINE_REQUEST_BOOT_INFO: RequestBootInfo;
    static LIMINE_REQUEST_TERMINAL: RequestTerminal;
}

lazy_static! {
    // handles concurrent terminal.write() calls
    static ref TERM0: Mutex<TerminalWriter> =
        Mutex::new(TerminalWriter::new(0).expect("Could not open limine terminal"));
}

// public interface to print to TERM0
// accepts non ASCII (non utf8 strings) -> b"Hello"
pub fn print0(s: &[u8]) {
    let access = TERM0.lock();
    ((access).write)(access.get_terminal(), s, s.len());
}

// ======= Boot Info feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#terminal-feature

// returns the bootloaders -> (name, version)
// WARNING: assumes the response has static lifetime
pub fn bootloader_info() -> (&'static [u8], &'static [u8]) {
    let response = unsafe { &*(LIMINE_REQUEST_BOOT_INFO.response) };
    (
        // SAFETY: ptr must not be null and must hold null terminated string
        unsafe { CStr::from_ptr(response.name as *const i8).to_bytes() },
        unsafe { CStr::from_ptr(response.version as *const i8).to_bytes() },
    )
}

#[repr(C)]
struct RequestBootInfo {
    id: [u64; 4],
    revision: u64,
    response: Ptr<ResponseBootInfo>,
}

#[repr(C)]
pub struct ResponseBootInfo {
    revision: u64,
    pub name: *const u8,
    pub version: *const u8,
}

// ======= Terminal feature
// See: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#bootloader-info-feature

struct TerminalWriter {
    term: usize, // pointer to terminal
    write: TerminalWriteFunction,
}

// handles 
impl TerminalWriter {
    fn new(terminal_number: u64) -> Option<Self> {
        let term_resp = unsafe { &*(LIMINE_REQUEST_TERMINAL.response) };
        if term_resp.terminal_count > terminal_number {
            return Some(Self {
                term: term_resp.terminals as usize + terminal_number as usize,
                write: term_resp.write,
            });
        }
        None
    }

    fn get_terminal(&self) -> *const Terminal {
        self.term as *const Terminal
    }
}

#[repr(C)]
struct RequestTerminal {
    id: [u64; 4],
    revision: u64,
    response: Ptr<ResponseTerminal>,
    callback: TerminalCallbackFunction,
}

#[repr(C)]
struct ResponseTerminal {
    revision: u64,
    terminal_count: u64,
    terminals: Ptr<Ptr<Terminal>>,
    write: TerminalWriteFunction,
}

#[repr(C)]
struct Terminal {
    columns: u64,
    rows: u64,
    framebuffer: Ptr<Framebuffer>,
}

// use by the Terminal feature and the Framebuffer feature
#[repr(C)]
struct Framebuffer {
    pub address: MutPtr<u8>,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub reserved: [u8; 7],
    pub edid_size: u64,
    pub edid: Ptr<u8>,
}
