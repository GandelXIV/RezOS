use std::collections::HashMap;
use std::env;
use std::fmt::Display;


pub const DEFAULT_BOOTLOADER:  &str     = "boot.bin";
pub const DEFAULT_OUTPUT:      &str     = "image.bin";
pub const DEFAULT_SOURCE:      &str     = "kernel.bin";
pub const DEFAULT_DIRECTBOOT:  bool     = true;
pub const DEFAULT_BLOCK_SIZE:  usize    = 512;
pub const DEFAULT_BLOCK_COUNT: AddrSize = AddrSize::Normal;

pub enum AddrSize {
    Normal, // 32 bit
    Large,  // 64 bit 
    Small,  // 16 bit
}

#[derive(PartialEq)]
pub enum Target {
    File(String),
    Dir(Vec<Target>),
    Raw(Vec<u8>),
}

pub struct Config {
    pub bootloader: Target,
    pub output: Target,
    pub source: Target,
    pub directboot: bool,
    pub block_size: usize,
    pub block_count: AddrSize, // normal: 4e9, large: 1.8e19, small: 6.5e4
}

impl Config {
    pub fn default() -> Self {
        Self {
            bootloader:  Target::File(String::from(DEFAULT_BOOTLOADER)),
            output:      Target::File(String::from(DEFAULT_OUTPUT)),
            source:      Target::File(String::from(DEFAULT_SOURCE)),
            directboot:  DEFAULT_DIRECTBOOT,
            block_size:  DEFAULT_BLOCK_SIZE,
            block_count: DEFAULT_BLOCK_COUNT
        }
    }

    pub fn argload() -> Self {
        let mut cfg = Self::default();
        let mut last = String::new();
        for arg in env::args() {
            match last.as_str() {
                "-b" => cfg.bootloader = Target::File(arg.clone()) ,
                "-o" => cfg.output = Target::File(arg.clone()),
                "-s" => cfg.source = Target::File(arg.clone()),
                "--directboot" => cfg.directboot = true,
                "--no-directboot" => cfg.directboot = false,
                "--block_count" => {
                    match arg.as_str() {
                        "32" => cfg.block_count = AddrSize::Normal,
                        "64" => cfg.block_count = AddrSize::Large,
                        "16" => cfg.block_count = AddrSize::Small,
                        _ => panic!("Invalid!"),
                    }
                }
                "--block_size" => cfg.block_size = arg.as_str().parse().unwrap(),
                _ => {}
            }
            last = arg;
        }
        cfg
    }
}
