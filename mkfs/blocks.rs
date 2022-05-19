use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SECTOR_SIZE;

// written in the SB for validation
pub const SIGN_SB: u16 = 0x4321;
pub type Addr = u32;

// used to store dynamically allocated blcoks on the disk
#[repr(C)]
pub union Node<'a> {
    pub dnode: &'a [u8],
    pub inode: &'a Inode,
}

// represents an array of disk sectors
#[derive(Serialize, Deserialize, new, Clone, Copy, Debug)]
pub struct Cluster {
    pub start: Addr,
    pub end: Addr,
}

// root block, contains info about the fs
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

// holds info about a file such as its meta-data and contents
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Inode {
    name: [char; SECTOR_SIZE / 16],       // file name
    flt: [Addr; SECTOR_SIZE / 16],        // front-link table
    blt: [Addr; SECTOR_SIZE / 16],        // back-link table
    pub dat: [Cluster; SECTOR_SIZE / 32], // dat=data allocation table
}

impl Inode {
    pub fn new() -> Self {
        Self {
            name: ['\u{0}'; SECTOR_SIZE / 16],
            flt: [0; SECTOR_SIZE / 16],
            blt: [0; SECTOR_SIZE / 16],
            dat: [Cluster::new(0, 0); SECTOR_SIZE / 32],
        }
    }

    pub fn name(&mut self, title: &str) {
        for (i, chr) in title.chars().enumerate() {
            self.name[i] = chr;
        }
    }
}
