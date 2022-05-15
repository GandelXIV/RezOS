// ENTFS => Entity file system; an entity is a file, a directory and a symlink at the same time
use std::fs::File;
use std::io::{Read, Write};
use bincode;

mod blocks;
mod config;


fn build_image(blocks: usize, blocksize: usize) -> Vec<Vec<u8>> {
    let mut image = vec![];
    for _ in 1..blocks {
        image.push(vec![0; blocksize]);
    }
    image
}

#[derive(Debug)]
enum MkfsError {
    BadConfig,
}

struct MkfsReport {}

fn mkfs(cfg: config::Config) -> Result<MkfsReport, MkfsError> {
    let mut image: Vec<Vec<u8>> = build_image(1000, cfg.block_size as usize);

    let sb = blocks::SuperBlock::new(1, cfg.block_size);
    image[1] = bincode::serialize(&sb).unwrap();

    match cfg.bootloader {
        config::Target::File(name) => {
            if File::open(name).unwrap().read(&mut image[0]).unwrap() > cfg.block_size as usize {
                panic!("Bootloader exceeds block size of {}", cfg.block_size);
            }
        },
        config::Target::Raw(data) => image[0] = data,
        _ => return Err(MkfsError::BadConfig), 
    }

    match cfg.output {
        config::Target::File(name) => {
            let mut compact = vec![];
            for mut block in image {
                compact.append(&mut block);
            }
            File::create(&name).unwrap().write(&compact).unwrap();
        }
        config::Target::Raw(_) => {
            for block in image {
                println!("{:?}", block);
            }
        }
        _ => return Err(MkfsError::BadConfig),
    }
    Ok(MkfsReport{})
}

fn main() {
    let cfg = config::Config::argload();
    mkfs(cfg).unwrap();
}
