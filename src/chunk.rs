use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug)]
enum ChunkError {
    CRCError,
    ChunkTypeError,
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::CRCError => write!(f, "CRC is wrong value."),
            ChunkError::ChunkTypeError => write!(f, "Chunk type is wrong value."),
        }
    }
}

impl std::error::Error for ChunkError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(value);
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        reader.read_exact(&mut buf)?;
        let chunk_type = match ChunkType::try_from(buf) {
            Ok(chunk_type) => chunk_type,
            Err(_) => return Err(Box::new(ChunkError::ChunkTypeError)),
        };

        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data)?;

        // CSC algorithm CRC-32/ISO-HDLC
        // width=32 poly=0x04c11db7 init=0xffffffff refin=true refout=true xorout=0xffffffff check=0xcbf43926 residue=0xdebb20e3 name="CRC-32/ISO-HDLC"
        reader.read_exact(&mut buf)?;
        let crc = u32::from_be_bytes(buf);
        let crc_checker = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let crc_value = crc_checker.checksum(&value[4..(4 + 4 + length) as usize]);

        if crc != crc_value {
            return Err(Box::new(ChunkError::CRCError));
        }

        let res = Self {
            length,
            chunk_type,
            data,
            crc,
        };
        println!("{:?}", res);
        Ok(res)
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let bytes = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = crc.checksum(&bytes);
        Self {
            length,
            chunk_type,
            data,
            crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String> {
        let mut res = String::with_capacity(self.length as usize);
        for &c in self.data.iter() {
            res.push(char::try_from(c)?);
        }
        Ok(res)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let length = self.length.to_be_bytes();
        let chunk_type = self.chunk_type.bytes();
        let data = self.data.as_slice();
        let crc = self.crc.to_be_bytes();
        length
            .iter()
            .chain(chunk_type.iter())
            .chain(data.iter())
            .chain(crc.iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        //let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
        let chunk = Chunk::try_from(chunk_data.as_ref());
        let chunk = chunk.unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        // let _chunk_string = format!("{}", chunk);
    }
}
