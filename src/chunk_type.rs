use std::{fmt::Display, str::FromStr};

fn is_latin_alphabetic(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn is_only_latin_alphabetic(s: &str) -> bool {
    s.chars().all(is_latin_alphabetic)
}

#[derive(Debug, Eq, PartialEq)]
pub struct ChunkType(pub [u8; 4]);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();
    /// 将传入的 `[u8; 4]` 类型数据包装为 `ChunkType([u8; 4])`
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType(value))
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    /// 将长度为 4 的字符串转换为 `ChunkType([u8; 4])`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err("String must be exactly 4 characters long")
        } else if !is_only_latin_alphabetic(s) {
            Err("String must contain only letters")
        } else {
            let bytes = s.as_bytes();
            println!("bytes: {:?}", bytes);
            if !bytes.iter().all(|&b| b.is_ascii()) {
                return Err("String must contain only ASCII characters");
            }
            let mut type_code = [0; 4];
            s.as_bytes()
                .iter()
                .zip(type_code.iter_mut())
                .for_each(|(b, tc)| *tc = *b);
            Ok(ChunkType(type_code))
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_code_str = std::str::from_utf8(&self.0).unwrap();
        write!(f, "{}", type_code_str)
    }
}

#[allow(dead_code)]
impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.0
    }
    fn is_valid(&self) -> bool {
        self.0 == [82, 117, 83, 116]
    }
    fn is_critical(&self) -> bool {
        self.0[0] == 82
    }
    fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
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
