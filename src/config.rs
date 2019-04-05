use std::io;
use std::io::{Read, Write, ErrorKind};

/* The magic number that leads every config file. */
const MAGIC_NUMBER: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
const LATEST_VERSION: u16 = 1u16;

pub struct TerribleConfig {
    version: u16,
    name: [u8; 64],
}

pub const fn new() -> TerribleConfig {
    TerribleConfig {
        version: 1u16,
        name: [0u8; 64]
    }
}

pub fn with_name(name: &str) -> TerribleConfig {
    let mut cfg = TerribleConfig {
        version: 1u16,
        name: [0u8; 64]
    };
    let bytes = name.as_bytes();
    let bytes_len = bytes.len();
    if bytes_len < 64 {
        cfg.name[0..bytes_len].copy_from_slice(bytes);
    }
    cfg
}

pub fn load(source: &mut Read) -> io::Result<TerribleConfig> {
    let mut buf = [0u8; 64];
    source.read_exact(&mut buf[0..4])?;
    if buf[0..4] != MAGIC_NUMBER {
        return Err(io::Error::new(ErrorKind::InvalidData, "magic number mismatch"))
    }
    source.read_exact(&mut buf[0..2])?;
    let version = u16::from_be_bytes([buf[0],buf[1]]);
    match version {
        1 => load_v1(source),
        _ => Err(io::Error::new(ErrorKind::InvalidData, "unknown config version")),
    }
}

fn load_v1(source: &mut Read) -> io::Result<TerribleConfig> {
    let mut cfg = TerribleConfig {
        version: 1u16,
        name: [0u8; 64],
    };
    source.read_exact(&mut cfg.name)?;
    Ok(cfg)
}

pub fn save(cfg: &TerribleConfig, destination: &mut Write) -> io::Result<()> {
    destination.write(&MAGIC_NUMBER)?;
    destination.write(&LATEST_VERSION.to_be_bytes())?;
    destination.write(&cfg.name)?;
    Ok(())
}
