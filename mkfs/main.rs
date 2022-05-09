// ENTFS => Entity file system; an entity is a file, a directory and a symlink at the same time
use std::fs::File;
use std::io::{Read, Write};
mod config;

fn build_image(blocks: usize, blocksize: usize) -> Vec<Vec<u8>> {
    let mut image = vec![];
    for _ in 1..blocks {
        image.push(vec![0; blocksize]);
    }
    image
}

fn main() {
    let cfg = config::Config::argload();
    let mut image: Vec<Vec<u8>> = build_image(1000, cfg.block_size);

    match cfg.bootloader {
        config::Target::File(name) => {
            if File::open(name).unwrap().read(&mut image[0]).unwrap() > cfg.block_size {
                panic!("Bootloader exceeds block size of {}", cfg.block_size);
            }
        },
        config::Target::Raw(data) => image[0] = data,
        _ => panic!("Bad config"),
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
        _ => panic!("Bad config"),
    }
}
