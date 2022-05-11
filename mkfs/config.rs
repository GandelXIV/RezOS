use std::collections::HashMap;
use std::env;
use std::fmt::Display;

#[allow(dead_code)]

pub const DEFAULT_BOOTLOADER:  &str     = "boot.bin";
pub const DEFAULT_OUTPUT:      &str     = "image.bin";
pub const DEFAULT_SOURCE:      &str     = "kernel.bin";
pub const DEFAULT_DIRECTBOOT:  bool     = true;
pub const DEFAULT_BLOCK_SIZE:  usize    = 512;


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
}

impl Config {
    pub fn default() -> Self {
        Self {
            bootloader:  Target::File(String::from(DEFAULT_BOOTLOADER)),
            output:      Target::File(String::from(DEFAULT_OUTPUT)),
            source:      Target::File(String::from(DEFAULT_SOURCE)),
            directboot:  DEFAULT_DIRECTBOOT,
            block_size:  DEFAULT_BLOCK_SIZE,
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
                "--block_size" => cfg.block_size = arg.as_str().parse().unwrap(),
                _ => {}
            }
            last = arg;
        }
        cfg
    }
}
