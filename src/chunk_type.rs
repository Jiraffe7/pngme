use core::str;
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

use crate::Error;

#[derive(Debug, PartialEq)]
struct ChunkType {
    data: u32,
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.data.to_be_bytes()
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        //let mask: [u8; 4] = [0b00010000, 0, 0, 0]; // this also works
        //let mask = [32, 0, 0, 0];
        //let mask = u32::from_be_bytes(mask);
        //self.data & mask != mask
        u8::is_ascii_uppercase(&self.data.to_be_bytes()[0])
    }

    fn is_public(&self) -> bool {
        //let mask = [0, 32, 0, 0];
        //let mask = u32::from_be_bytes(mask);
        //self.data & mask != mask
        u8::is_ascii_uppercase(&self.data.to_be_bytes()[1])
    }

    fn is_reserved_bit_valid(&self) -> bool {
        //let mask = [0, 0, 32, 0];
        //let mask = u32::from_be_bytes(mask);
        //self.data & mask != mask
        u8::is_ascii_uppercase(&self.data.to_be_bytes()[2])
    }

    fn is_safe_to_copy(&self) -> bool {
        //let mask = [0, 0, 0, 32];
        //let mask = u32::from_be_bytes(mask);
        //self.data & mask == mask
        u8::is_ascii_lowercase(&self.data.to_be_bytes()[3])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if !bytes.iter().all(u8::is_ascii_alphabetic) {
            return Err("bytes are not ASCII alphabetic".into());
        }
        let data = u32::from_be_bytes(bytes);
        let chunk_type = ChunkType { data };
        Ok(chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err("str is not ASCII".into());
        }
        let b = s.as_bytes();
        let bytes: [u8; 4] = match b.try_into() {
            Ok(arr) => arr,
            Err(_) => Err("length of vec greater than 4")?,
        };
        //let chars: Vec<_> = s.chars().map(u8::try_from).map(Result::unwrap).collect();
        //let bytes: [u8; 4] = match chars.try_into() {
        //    Ok(arr) => arr,
        //    Err(_) => Err("length of vec greater than 4")?,
        //};
        let chunk_type: ChunkType = bytes.try_into()?;
        Ok(chunk_type)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.data.to_be_bytes();
        let s = match str::from_utf8(&b) {
            Ok(s) => s,
            Err(_) => Err(std::fmt::Error)?,
        };
        //let s: String = self
        //    .data
        //    .to_be_bytes()
        //    .map(char::from)
        //    .into_iter()
        //    .collect();
        write!(f, "{s}")
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
