use crate::Result;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug)]
pub struct ChunkType {
    codes: [u8; 4],
}

#[derive(Debug, Error)]
enum ChunkTypeError {
    #[error("using Reserved Bit")]
    ReservedBit,
    #[error("")]
    InvalidByte,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        let res = Self { codes: value };
        if !res.is_reserved_bit_valid() {
            Err(ChunkTypeError::ReservedBit.into())
        } else if !res.is_only_alphabetic() {
            Err(ChunkTypeError::InvalidByte.into())
        } else {
            Ok(res)
        }
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().all(|c| c.is_alphabetic()) {
            let bytes = s.as_bytes();
            let codes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
            let chunktype = Self { codes };
            Ok(chunktype)
        } else {
            Err(ChunkTypeError::InvalidByte.into())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let codes_str = String::from_utf8(self.codes.to_vec()).unwrap();
        write!(f, "{}", codes_str)
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &ChunkType) -> bool {
        self.codes
            .iter()
            .zip(other.codes.iter())
            .all(|(c1, c2)| c1 == c2)
    }
}

impl Eq for ChunkType {}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.codes
    }
    pub fn is_valid(&self) -> bool {
        self.is_only_alphabetic() && self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        self.codes[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.codes[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.codes[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.codes[3].is_ascii_lowercase()
    }
    fn is_only_alphabetic(&self) -> bool {
        self.codes.iter().all(|byte| byte.is_ascii_alphabetic())
    }
}

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
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
