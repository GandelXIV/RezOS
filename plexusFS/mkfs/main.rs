use anyhow::{anyhow, Result as AnyhowResult};
use derive_new::new;
use std::io::{Read, Write};
use std::mem::size_of;
use std::ops::Add;
use std::path::{self, Path, PathBuf};
use std::ptr;
use std::{any, fs};

type Addr = u32; // LBA addressing for disk sectors

const SECTOR_SIZE: usize = 512; // same as block size

struct Config {
    input: String,  // input dir path
    output: String, // output file path
    max_file_count: Addr,   // size of FAT
    init_file_size: usize,  // unused
}

impl Config {
    fn default() -> Self {
        Self {
            input: String::from("../initrd/"),
            output: String::from("../build/initrd.bin"),
            max_file_count: 1024,
            init_file_size: SECTOR_SIZE,
        }
    }
}

// represents a memory area
#[derive(new, Clone, Copy, Debug)]
struct Chunk {
    start: Addr,
    end: Addr,
}

impl Chunk {
    fn null() -> Self {
        Self { start: 0, end: 0 }
    }
}

const HEAD_MAGIC_SIZE: usize = 4;
const HEAD_PADDDING_SIZE: usize = 478;
const HEAD_DAT_MAX_SIZE: Addr = 10000;

// equivalent to the Super Block in other FS
#[repr(C, packed)]
struct Head {
    version: u16,                   // 1
    magic: [char; HEAD_MAGIC_SIZE], // always 'head'
    fat: Chunk,                     // = file allocation table
    dat: Chunk,                     // = data alloaction table
    _pad: [u8; HEAD_PADDDING_SIZE], // void
}

const INODE_NAME_SIZE: usize = 64;
const INODE_FLT_SIZE: usize = 50;
const INODE_BLT_SIZE: usize = 50;
const INODE_FRAGS_SIZE: usize = 6;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
struct Inode {
    name: [u8; INODE_NAME_SIZE],      // file name
    flt: [Addr; INODE_FLT_SIZE],      // front link table [subfiles]
    blt: [Addr; INODE_BLT_SIZE],      // back link table [supfiles]
    frags: [Chunk; INODE_FRAGS_SIZE], // data fragments [content]
}

impl Inode {
    fn new(name: &[u8]) -> Self {
        let mut instance = Self {
            name: [0; INODE_NAME_SIZE],
            flt: [0; INODE_FLT_SIZE],
            blt: [0; INODE_BLT_SIZE],
            frags: [Chunk::null(); INODE_FRAGS_SIZE],
        };
        instance.name[..name.len()].copy_from_slice(&name[..]);
        instance
    }
}

// read T as raw bin
fn conver2sector<T>(n: T) -> Vec<u8> {
    assert_eq!(std::mem::size_of::<T>(), SECTOR_SIZE);  // safety check
    Vec::from(unsafe { *(ptr::addr_of!(n) as *const [u8; SECTOR_SIZE]) })
}

// main function
fn mkfs(cfg: &Config) -> Vec<u8> {
    // check if components have exact size of disk sector
    assert_eq!(std::mem::size_of::<Inode>(), SECTOR_SIZE);
    assert_eq!(std::mem::size_of::<Head>(), SECTOR_SIZE);

    // init sections
    let head = Head {
        version: 1,
        magic: ['h', 'e', 'a', 'd'],
        fat: Chunk::new(1, cfg.max_file_count),
        dat: Chunk::new(
            cfg.max_file_count + 1,
            cfg.max_file_count + HEAD_DAT_MAX_SIZE + 1,
        ),
        _pad: [0; HEAD_PADDDING_SIZE],
    };
    let mut fat = vec![Inode::new(&[b'/'])];    // empty fat with the apex(root)
    let mut dat: Vec<u8> = Vec::new();

    // writing files from input > target (no recusion support currently)
    for (fid, entry) in fs::read_dir(&cfg.input).unwrap().enumerate() {
        let path = entry.unwrap().path();

        let mut inode = Inode::new(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .as_bytes(),
        );

        let mut content = Vec::new();
        fs::File::open(path)
            .unwrap()
            .read_to_end(&mut content)
            .unwrap();

        // align data to SECTOR_SIZE by appending an empty buffer to content
        content.append(&mut vec![0_u8; SECTOR_SIZE - content.len() % SECTOR_SIZE]);

        // link to apex
        inode.blt[0] = 1; // back link current file to the apex (root)
        fat[0].flt[fid] = fid as u32 + 2; // front link apex to current file

        // write data fragment
        let content_size = content.len() / SECTOR_SIZE;
        let content_start = head.dat.start + (dat.len() / SECTOR_SIZE) as Addr;
        inode.frags[0] = Chunk::new(content_start, content_start + content_size as Addr - 1);
        dat.append(&mut content);

        fat.push(inode);
    }

    // build the image
    let mut image: Vec<u8> = Vec::new();

    image.append(&mut conver2sector(head));
    for inode in fat {
        image.append(&mut conver2sector(inode));
    }
    
    image.append(&mut dat);

    image
}

fn main() {
    let cfg = Config::default();
    let image = mkfs(&cfg);
    fs::File::create(cfg.output)
        .unwrap()
        .write_all(&image)
        .unwrap();

    println!("Done!");
}
