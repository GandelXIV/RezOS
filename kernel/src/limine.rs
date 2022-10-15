use core::ptr::NonNull;
use core::str;

const MAGIC_COMMON: (u64, u64) = (0xc7b1dd30df4c8b88, 0x0a82e883a194f07b);
type Ptr<T> = *const T;
type MutPtr<T> = *mut T;
type TerminalCallback = extern "C" fn(Ptr<Terminal>, u64, u64, u64, u64);

pub fn term_write(txt: &str) {
    let term_resp = unsafe { &*(LIMINE_REQUEST_TERMINAL.response) };
    if term_resp.terminal_count > 0 {
        let term = unsafe { &**(term_resp.terminals) };
        unsafe {
            (term_resp.write)(term, txt, txt.len());
        }
    }
}

pub fn init() {
    term_write("Hello from limine terminal!\n");
}

// TODO: process these requests
extern "C" {
    static LIMINE_REQUEST_BOOT_INFO: RequestBootInfo;
    static LIMINE_REQUEST_TERMINAL: RequestTerminal;
}

// Boot Info feature

#[repr(C)]
struct RequestBootInfo {
    id: [u64; 4],
    revision: u64,
    response: Ptr<ResponseBootInfo>,
}

#[repr(C)]
struct ResponseBootInfo {
    revision: u64,
    nameptr: Ptr<[u8]>,
    versionptr: Ptr<[u8]>,
}

// Terminal feature

#[repr(C)]
struct RequestTerminal {
    id: [u64; 4],
    revision: u64,
    response: Ptr<ResponseTerminal>,
    callback: TerminalCallback,
}

#[repr(C)]
struct ResponseTerminal {
    revision: u64,
    terminal_count: u64,
    terminals: Ptr<Ptr<Terminal>>,
    write: unsafe extern "C" fn(Ptr<Terminal>, &str, usize),
}

#[repr(C)]
struct Terminal {
    columns: u64,
    rows: u64,
    framebuffer: Ptr<Framebuffer>,
}

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
