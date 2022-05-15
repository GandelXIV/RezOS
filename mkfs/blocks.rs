use serde::{Serialize, Deserialize};
use derive_new::new;

pub const SIGN_SB: u32 = 564831;
pub const SIGN_INODE: u32 = 768323;
pub type Addr = u32;

pub enum BlockType {
    
}

#[derive(Serialize, Deserialize, new)]
pub struct Cluster {
    start: Addr,
    end: Addr,
}

#[derive(Serialize, Deserialize)]
pub struct SuperBlock {
    sign: u32,
    version: u16,
    blocksize: u16,
    directboot: Option<Cluster>,
    root: Option<Addr>,
}

#[derive(Serialize, Deserialize)]
pub struct Inode {
    sign: u32,
    name: Vec<char>,
    fat: Vec<Cluster>,
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

impl Inode {
    pub fn new(name: &str) -> Self {
        Self {
            sign: SIGN_INODE,
            name: name.chars().collect(),
            fat: vec![],
        }
    }
}