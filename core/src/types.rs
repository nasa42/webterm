use crate::generated::flatbuffers_schema::talk_v1::Bits256 as FbBits256;
use crate::generated::flatbuffers_schema::talk_v1::Bits96 as FbBits96;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionId(pub u64);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActivityId(pub u64);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrontendId(pub u64);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bits96(pub [u8; 12]);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bits256(pub [u8; 32]);

impl From<Bits96> for FbBits96 {
    fn from(val: Bits96) -> Self {
        FbBits96::new(&val.0)
    }
}

impl From<&FbBits96> for Bits96 {
    fn from(bits: &FbBits96) -> Self {
        Self(<[u8; 12]>::from(bits.bytes()))
    }
}

impl From<Bits256> for FbBits256 {
    fn from(val: Bits256) -> Self {
        FbBits256::new(&val.0)
    }
}

impl From<&FbBits256> for Bits256 {
    fn from(bits: &FbBits256) -> Self {
        Self(<[u8; 32]>::from(bits.bytes()))
    }
}

impl From<u64> for Bits96 {
    fn from(value: u64) -> Self {
        let mut data = [0u8; 12];
        data[4..].copy_from_slice(&value.to_be_bytes());
        Self(data)
    }
}

impl From<u64> for Bits256 {
    fn from(value: u64) -> Self {
        let mut data = [0u8; 32];
        data[24..].copy_from_slice(&value.to_be_bytes());
        Self(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits96_conversion() {
        let original = Bits96([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let fb: FbBits96 = original.into();
        let converted: Bits96 = (&fb).into();
        assert_eq!(original, converted);
    }

    #[test]
    fn test_bits256_conversion() {
        let mut data = [0u8; 32];
        for (i, element) in data.iter_mut().enumerate() {
            *element = i as u8;
        }
        let original = Bits256(data);
        let fb: FbBits256 = original.into();
        let converted: Bits256 = (&fb).into();
        assert_eq!(original, converted);
    }

    #[test]
    fn test_u64_to_bits_conversion() {
        let value: u64 = 0x1234567890ABCDEF;
        let bits96 = Bits96::from(value);
        let bits256 = Bits256::from(value);

        assert_eq!(
            bits96.0,
            [0x00, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]
        );

        assert_eq!(
            bits256.0,
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x34, 0x56, 0x78,
                0x90, 0xAB, 0xCD, 0xEF
            ]
        );
    }
}
