use std::collections::HashMap;
// ENTFS => Entity file system; an entity is a file, a directory and a symlink at the same time
use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::ops::Add;
use bincode;
use blocks::{SuperBlock, Inode, Addr};

mod blocks;
mod config;

const IMAGE_MAX_BLOCK_COUNT: usize = 9876;
const IMAGE_BOOT_ADDR: Addr = 0;
const IMAGE_SUPER_ADDR: Addr = 1;



#[derive(Debug)]
enum MkfsError {
    BadConfig,
    FileNotFound(String),
}

struct MkfsReport {}

struct Image {
    sb: SuperBlock,
    bin: HashMap<Addr, Vec<u8>>,
    inodes: HashMap<Addr, Inode>,
    bat: Vec<bool>, // block allocation table
}

impl Image {
    fn new(sb: SuperBlock, boot: Vec<u8>) -> Self {
        let mut instance = Self {
            sb: sb,
            bin: HashMap::new(),
            inodes: HashMap::new(),
            bat: vec![false; IMAGE_MAX_BLOCK_COUNT],
        };
        // asign boot
        instance.bat[IMAGE_BOOT_ADDR as usize] = true;
        instance.bin.insert(IMAGE_BOOT_ADDR, boot);
        // assign super
        instance.bat[IMAGE_SUPER_ADDR as usize] = true;
        // return
        instance
    }
    
    fn assign_block(&mut self) -> Result<Addr, anyhow::Error> {
        for i in 0..self.bat.len() {
            if !self.bat[i] {
                self.bat[i] = true;
                return Ok(i as Addr);
            }
        }
        Err(anyhow::anyhow!("Could not find free block"))
    }
    
    fn build(&self, target: &mut Vec<u8>) -> anyhow::Result<()>{
        for i in 0..self.bat.len() {
            if i as Addr == IMAGE_SUPER_ADDR {
                target.append(&mut bincode::serialize(&self.sb)?);
            }
            else if let Some(raw) = self.bin.get(&(i as Addr)).take() {
                target.append(&mut raw.clone());   // messy way to do it
            }
            else if let Some(inode) = self.inodes.get(&(i as Addr)).take() {
                target.append(&mut bincode::serialize(&inode)?);
            }
            else {
                target.append(&mut vec![0; self.sb.blocksize as usize]);
            }
        }
        Ok(())
    }
}

fn mkfs(cfg: config::Config) -> Result<MkfsReport, MkfsError> {
    let mut boot = Vec::new();
    match cfg.bootloader {
        config::Target::File(name) => {
            if let Ok(file) = File::open(&name) {
                let mut buf_reader = BufReader::new(file);
                buf_reader.read_to_end(&mut boot).unwrap();
            }
            else {
                return Err(MkfsError::FileNotFound(name));
            }
        },
        config::Target::Raw(data) => boot = data,
        _ => return Err(MkfsError::BadConfig), 
    }

    let image = Image::new( blocks::SuperBlock::new(1, cfg.block_size), boot );
    
    match cfg.output {
        config::Target::File(name) => {
            let mut compact = vec![];
            image.build(&mut compact).unwrap();
            File::create(&name).unwrap().write(&compact).unwrap();
        }
        _ => return Err(MkfsError::BadConfig),
    }
    Ok(MkfsReport{})
}

fn main() {
    let cfg = config::Config::argload();
    mkfs(cfg).unwrap();
}
