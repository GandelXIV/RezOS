use serde::{Serialize, Deserialize};
use derive_new::new;
use anyhow;

pub const SIGN_SB: u16 = 0x4321;
pub const SIGN_INODE: u16 = 0x1234;
pub const ALLOWED_FILE_NAME_CHARS: &'static str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890._";
pub type Addr = u32;


pub union Node<'a> {
    pub dnode: &'a [u8],
    pub inode: &'a Inode,
}


#[derive(Serialize, Deserialize, new)]
pub struct Cluster {
    start: Addr,
    end: Addr,
}

#[derive(Serialize, Deserialize)]
pub struct SuperBlock {
    pub sign: u16,
    pub version: u16,
    pub blocksize: u16,
    pub directboot: Option<Cluster>,
    pub root: Option<Addr>,
}

pub struct Inode {
    sign: u16,
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
    pub fn new(name: &str) -> Result<Self, anyhow::Error> {
        for i in name.chars() {
            let mut valid = false;
            for j in ALLOWED_FILE_NAME_CHARS.chars() {
                if i == j {
                    valid = true;
                }
            }
            if !valid {
                return Err(anyhow::anyhow!("invalid character '{}' in file name: {}", i, name));
            }
        }
        Ok(Self {
            sign: SIGN_INODE,
            name: name.chars().collect(),
            fat: vec![],
        })
    }
}