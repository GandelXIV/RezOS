use serde::{Serialize, Deserialize};
use derive_new::new;
use anyhow;

use crate::config::SECTOR_SIZE;

pub const SIGN_SB: u16 = 0x4321;
pub type Addr = u32;


pub union Node<'a> {
    pub dnode: &'a [u8],
    pub inode: &'a Inode,
}


#[derive(Serialize, Deserialize, new, Clone, Copy)]
pub struct Cluster {
    pub start: Addr,
    pub end: Addr,
}

#[derive(Serialize, Deserialize)]
pub struct SuperBlock {
    pub sign: u16,
    pub version: u16,
    pub blocksize: u16,
    pub directboot: Option<Cluster>,
    pub root: Option<Addr>,
}

impl SuperBlock {
    pub fn new(version: u16, blocksize: u16) -> Self {
        Self {
            sign: SIGN_SB,
            version: version,
            blocksize: blocksize,
            directboot: None,
            root: None,
        }
    }
}

#[repr(C)]
pub struct Inode {
    name: [char; SECTOR_SIZE/16],
    flt: [Addr; SECTOR_SIZE/16],
    blt: [Addr; SECTOR_SIZE/16],
    pub dat: [Cluster; SECTOR_SIZE/32],
}

impl Inode {
    pub fn new() -> Self {
        Self {
            name: ['\u{0}'; SECTOR_SIZE/16],
            flt: [0; SECTOR_SIZE/16],
            blt: [0; SECTOR_SIZE/16],
            dat: [Cluster::new(0, 0); SECTOR_SIZE/32],
        }
    }

    pub fn name(&mut self, title: &str) {
        for (i, chr) in title.chars().enumerate() {
            self.name[i] = chr;
        }
    }
}