use std::str::FromStr;
use std::{fmt, str};

#[derive(Debug)]
pub struct ChunkType {
    bytes: [u8; 4]
}

impl ChunkType {
    fn bytes(&self) -> [u8;4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        for byte in self.bytes {
            if !ChunkType::is_valid_byte(byte) {
                return false;
            }
        }
        return true;
    }

    fn is_err(&self) -> bool {
        !self.is_valid()
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        is_uppercase(self.bytes[0])
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        is_uppercase(self.bytes[1])
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        is_uppercase(self.bytes[2])
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        !is_uppercase(self.bytes[3])
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        if byte < 65 ||( 90 < byte && byte < 97) || 122 < byte {
            return false;
        }
        true
    }

    pub fn to_string(&self) -> &str {
        str::from_utf8(&self.bytes).unwrap()
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &ChunkType) -> bool {
        for (i, x) in self.bytes().iter().enumerate() {
            if x != &other.bytes()[i] {
                return false
            }
        }
        true
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err("Lenght of chunk byte vector needs to be 4");
        }
        let res = ChunkType {bytes: value};
        Ok(res)
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = value.as_bytes();
        if bytes.len() != 4 {
            return Err("Must contain exactly four ASCII letters!");
        }
        for byte in bytes {
            let ubyte = *byte;
            if ubyte < 65 ||( 90 < ubyte && ubyte < 97) || 122 < ubyte {
                return Err("All letters need to be valid ASCII chars!");
            }
        }
        let byte_array: [u8;4] = bytes.try_into().expect("slice with incorrect length!");
        let res = ChunkType{bytes:byte_array};
        Ok(res)		

    }
}

fn check_valid_ascii(ubyte: u8) -> bool {
    if ubyte < 65 ||( 90 < ubyte && ubyte < 97) || 122 < ubyte {
        return false;
    }
    true
}

fn is_uppercase(ubyte: u8) -> bool {
    if ubyte < 90 {
        return true;
    }
    false
}

// #![allow(unused_variables)]
// fn main() {
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        println!("{}", chunk.is_valid());
        assert!(chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
// }
