// use crate::Result;
use crate::chunk_type::ChunkType;
use std::convert::TryFrom;
use std::fmt;
use crc;

use crate::{Error, Result};

pub struct Chunk {
    data: Vec<u8>,
    chunk_type: ChunkType,
}

impl Chunk {
    pub fn length(&self) -> usize {
        self.data.len()
        // self.length.try_into().unwrap()
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    } 

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();
        crc::crc32::checksum_ieee(&bytes)
        // self.crc
    }
    
    pub fn data_as_string(&self) -> Result<String> {
        let data = self.data.clone();
        Ok(String::from_utf8(data).unwrap())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let len_bytes = (self.length() as u32).to_be_bytes();
        let bytes: Vec<u8> = len_bytes
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect();
        bytes
    }

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk{chunk_type: chunk_type, data: data}
    }

}

#[derive(Debug)]
pub enum CreateChunkError {
    ParseError,
    MismatchedCrc
}

impl std::error::Error for CreateChunkError {
    
}

impl fmt::Display for CreateChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CreateChunkError::ParseError => {
                write!(f, "Failed to parse chunk!")
            }
            CreateChunkError::MismatchedCrc => {
                write!(f, "CRC does not match calculated value!")
            }
        }
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        // todo!()
        let mut mut_value = value.clone();
        let split_1 = mut_value.split_at(4);
        let rem: [u8; 4] = (split_1.0).try_into().expect("Could not read size!");
        let length: usize = u32::from_be_bytes(rem) as usize;
        let split_2 = split_1.1.split_at(4);
        let chunk_rem: [u8; 4] = (split_2.0).try_into().expect("Could not read out chunk type!");
        let chunk_type = ChunkType::try_from(chunk_rem).unwrap();
        let split_3 = split_2.1.split_at(split_2.1.len() - 4);
        let data: Vec<u8> = split_3.0.to_vec();
        let crc: u32 = u32::from_be_bytes(split_3.1.try_into().expect("Could not read out crc!"));
        let res = Chunk{ chunk_type: chunk_type,  data: data};
        let expected_crc = res.crc();
        if crc != expected_crc {
            return Err(Box::from(CreateChunkError::MismatchedCrc));
        }
        Ok(res)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
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
         
        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
        print!("{}", chunk);
        chunk
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
        println!("{} vs {}", chunk_string, expected_chunk_string);
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

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

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
        let err = chunk.is_err();

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
        
        let _chunk_string = format!("{}", chunk);
    }

}
