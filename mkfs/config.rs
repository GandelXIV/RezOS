use std::env;

pub const DEFAULT_BOOTLOADER: &str = "build/boot.bin";
pub const DEFAULT_OUTPUT: &str = "build/RezOS.bin";
pub const DEFAULT_SOURCE: &str = "build/kernel.bin";
pub const DEFAULT_DIRECTBOOT: bool = true;
pub const DEFAULT_BLOCK_SIZE: u16 = 512;

// size of disk sectors, do not change!
pub const SECTOR_SIZE: usize = 512;

#[derive(PartialEq)]
pub enum Target {
    File(String),
    Dir(Vec<Target>),
    Raw(Vec<u8>),
}

pub struct Config {
    pub bootloader: Target, // stage 1 bootloader
    pub output: Target,     // currently can only be File
    pub source: Target,     // currently can only be the kernel
    pub directboot: bool,   // creates a link to kernel in the superblock, required for current version of the bootloader
    pub block_size: u16,    // unused setting
}

// holds configuration for mkfs()
impl Config {
    pub fn default() -> Self {
        Self {
            bootloader: Target::File(String::from(DEFAULT_BOOTLOADER)),
            output: Target::File(String::from(DEFAULT_OUTPUT)),
            source: Target::File(String::from(DEFAULT_SOURCE)),
            directboot: DEFAULT_DIRECTBOOT,
            block_size: DEFAULT_BLOCK_SIZE,
        }
    }

    // load config from cmd line arguments
    pub fn argload() -> Self {
        let mut cfg = Self::default();
        let mut last = String::new();
        for arg in env::args() {
            match last.as_str() {
                "-b" => cfg.bootloader = Target::File(arg.clone()),
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
