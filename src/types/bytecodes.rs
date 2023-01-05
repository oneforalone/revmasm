use std::{
    borrow::Borrow,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use bytes::Bytes;
#[derive(Clone, Default)]
pub struct Bytecodes(pub bytes::Bytes);

pub fn bytecodes_to_hex(bc: &Bytecodes) -> String {
    hex::encode(bc.0.as_ref())
}

impl Debug for Bytecodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Bytecodes(0x{})", bytecodes_to_hex(self))
    }
}

impl Display for Bytecodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{}", bytecodes_to_hex(self))
    }
}

impl Bytecodes {
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_ref().to_vec()
    }
}

impl Deref for Bytecodes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsRef<[u8]> for Bytecodes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Borrow<[u8]> for Bytecodes {
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl IntoIterator for Bytecodes {
    type Item = u8;
    type IntoIter = bytes::buf::IntoIter<bytes::Bytes>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Bytecodes {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}

impl From<bytes::Bytes> for Bytecodes {
    fn from(b: bytes::Bytes) -> Self {
        Self(b)
    }
}

impl From<Vec<u8>> for Bytecodes {
    fn from(v: Vec<u8>) -> Self {
        Self(v.into())
    }
}

impl From<String> for Bytecodes {
    fn from(s: String) -> Self {
        let hex_str = hex::decode(s).expect("String -> HexString Decode Failed");
        Self(Bytes::from(hex_str))
    }
}
