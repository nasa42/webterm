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

impl Into<FbBits96> for Bits96 {
    fn into(self) -> FbBits96 {
        FbBits96::new(&self.0)
    }
}

impl From<&FbBits96> for Bits96 {
    fn from(bits: &FbBits96) -> Self {
        Self(<[u8; 12]>::from(bits.bytes()))
    }
}

impl Into<FbBits256> for Bits256 {
    fn into(self) -> FbBits256 {
        FbBits256::new(&self.0)
    }
}

impl From<&FbBits256> for Bits256 {
    fn from(bits: &FbBits256) -> Self {
        Self(<[u8; 32]>::from(bits.bytes()))
    }
}
