// ENTFS => Entity file system; an entity is a file, a directory and a symlink at the same time
use bincode;
use blocks::{Addr, Inode, Node, SuperBlock};
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::mem::size_of;
use std::cell::RefCell;

use crate::blocks::Cluster;
use crate::config::SECTOR_SIZE;

mod blocks;
mod config;

#[derive(Debug)]
enum MkfsError {
    BadConfig,
    FileNotFound(String),
}

struct MkfsReport {}

struct Image<'b> {
    sb: SuperBlock,
    boot: Vec<u8>,
    nodes: Vec<Node<'b>>,
}

impl<'b> Image<'b> {
    fn new(sb: SuperBlock, boot: Vec<u8>) -> Self {
        Self {
            sb: sb,
            boot: boot,
            nodes: vec![],
        }
    }

    fn build(&mut self, target: &mut Vec<u8>) {
        target.append(&mut self.boot);
        target.append(&mut bincode::serialize(&self.sb).unwrap());
        for node in self.nodes.iter() {
            unsafe {
                for i in 0..SECTOR_SIZE {
                    target.push(node.dnode[i]);
                }
            }
        }
    }
}

fn mkfs(cfg: config::Config) -> Result<MkfsReport, MkfsError> {
    let mut boot = Vec::new();
    match cfg.bootloader {
        config::Target::File(name) => {
            if let Ok(file) = File::open(&name) {
                let mut buf_reader = BufReader::new(file);
                buf_reader.read_to_end(&mut boot).unwrap(); // lmao
            } else {
                return Err(MkfsError::FileNotFound(name));
            }
        }
        config::Target::Raw(data) => boot = data,
        _ => return Err(MkfsError::BadConfig),
    }

    let mut image = Image::new(blocks::SuperBlock::new(1, cfg.block_size), boot);

    assert_eq!(size_of::<Inode>(), SECTOR_SIZE);

    let mut inode_container = vec![];
    let mut dnode_container = vec![];
    match cfg.source {
        config::Target::File(name) => {
            // prep data
            let mut content = Vec::new();
            if let Ok(file) = File::open(&name) {
                let mut buf_reader = BufReader::new(file);
                { buf_reader.read_to_end(&mut content).unwrap(); }
            } else {
                return Err(MkfsError::FileNotFound(name));
            }
            let location = Cluster::new(1, (content.len() / SECTOR_SIZE + content.len() % SECTOR_SIZE) as Addr);
            // setup inode
            let mut inode = Inode::new();
            inode.name(&name);
            inode.dat[0] = location.clone();
            inode_container.push(inode);
            image.nodes.push( Node{ inode: &inode_container[0] } );
            // load data
            for i in location.start .. location.end+1 {
                if content.len() >= SECTOR_SIZE {
                    dnode_container.push( content.drain(0..SECTOR_SIZE).collect::<Vec<u8>>() );
                } else {
                    let mut v = vec![0u8; SECTOR_SIZE];
                    for (i, b) in content.drain(0..content.len()).enumerate() {
                        v[i] = b;
                    }
                    dnode_container.push( v );
                    
                }
            }
            for d in &dnode_container {
                image.nodes.push(Node{ dnode: d });
            }
        }
        _ => return Err(MkfsError::BadConfig),
    }

    match cfg.output {
        config::Target::File(name) => {
            let mut compact = vec![];
            image.build(&mut compact);
            File::create(&name).unwrap().write(&compact).unwrap();
        }
        _ => return Err(MkfsError::BadConfig),
    }
    Ok(MkfsReport {})
}

fn main() {
    let cfg = config::Config::argload();
    mkfs(cfg).unwrap();
}
