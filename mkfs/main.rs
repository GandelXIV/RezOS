// ENTFS => Entity file system; an entity is a file, a directory and a symlink at the same time
use bincode;
use blocks::{Addr, Inode, Node, SuperBlock};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::mem::size_of;
use std::path::Path;

use crate::blocks::Cluster;
use crate::config::SECTOR_SIZE;

mod blocks;
mod config;

#[derive(Debug)]
enum MkfsError {
    BadConfig,
    FileNotFound(String),
    EmptyBootloader,
    InvalidInode(usize),
}

struct MkfsReport {
    fssize: usize, // in bytes
    inode_count: usize,
    dnode_count: usize,
}

impl Display for MkfsReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[MKFS REPORT]\nSize: {}B\nInode count:{}\nDatanode count:{}\n",
            self.fssize, self.inode_count, self.dnode_count
        )
    }
}

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
        for (_dbg, node) in self.nodes.iter().enumerate() {
            unsafe {
                for i in 0..SECTOR_SIZE {
                    target.push(node.dnode[i]);
                }
            }
        }
    }
}

// main function
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

    if boot == vec![] {
        return Err(MkfsError::EmptyBootloader);
    }

    let mut image = Image::new(blocks::SuperBlock::new(1, cfg.block_size), boot);

    if size_of::<Inode>() != SECTOR_SIZE {
        return Err(MkfsError::InvalidInode(size_of::<Inode>()));
    }

    let mut inode_container = vec![];
    let mut dnode_container = vec![];
    match cfg.source {
        config::Target::File(name) => {
            // prep data
            let mut content = Vec::new();
            if let Ok(file) = File::open(&name) {
                let mut buf_reader = BufReader::new(file);
                {
                    buf_reader.read_to_end(&mut content).unwrap();
                }
            } else {
                return Err(MkfsError::FileNotFound(name));
            }
            let location = Cluster::new(
                1,
                (content.len() / SECTOR_SIZE + (content.len() % SECTOR_SIZE > 0) as usize) as Addr,
            );
            // extract name from path
            let name = Path::new(&name).file_name().unwrap().to_str().unwrap();
            // directboot
            if cfg.directboot && name == config::DIRECT_BOOT_TARGET {
                image.sb.directboot = Some(location);
            }
            // setup inode
            let mut inode = Inode::new();
            inode.name(&name);
            inode.dat[0] = location.clone();
            inode_container.push(inode);
            image.nodes.push(Node {
                inode: &inode_container[0],
            });
            // load data
            for i in location.start..location.end + 1 {
                if content.len() >= SECTOR_SIZE {
                    dnode_container.push(content.drain(0..SECTOR_SIZE).collect::<Vec<u8>>());
                } else {
                    let mut v = vec![0u8; SECTOR_SIZE];
                    for (i, b) in content.drain(0..content.len()).enumerate() {
                        v[i] = b;
                    }
                    dnode_container.push(v);
                }
            }
            for d in &dnode_container {
                image.nodes.push(Node { dnode: d });
            }
        }
        _ => return Err(MkfsError::BadConfig),
    }

    let mut compact = vec![];
    match cfg.output {
        config::Target::File(name) => {
            image.build(&mut compact);
            File::create(&name).unwrap().write(&compact).unwrap();
        }
        _ => return Err(MkfsError::BadConfig),
    }
    Ok(MkfsReport {
        fssize: compact.len(),
        dnode_count: dnode_container.len(),
        inode_count: inode_container.len(),
    })
}

fn main() {
    let cfg = config::Config::argload();
    let report = mkfs(cfg).unwrap();
    println!("{}", report);
}
