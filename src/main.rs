use std::str::FromStr;

use chunk::Chunk;
use chunk_type::ChunkType;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // let chunk = ChunkType::from_str("Rust").unwrap();
    // println!("{}", chunk);
    // const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    // let crc = X25.checksum(b"2882656334");
    // print!("{}", crc);
    let chunk_type = ChunkType::from_str("RuSt").unwrap();
    let data = "This is where your secret message will be!"
        .as_bytes()
        .to_vec();
    let chunk = Chunk::new(chunk_type, data);
    println!("chunk: {:?}", chunk);
    Ok(())
}
