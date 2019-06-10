pub mod de;

use ethereum_types::U256;
use indexmap::IndexMap;
use ssz::{Encode, Output, Prefixable};

#[derive(Clone, Debug, PartialEq)]
pub enum SsztValue {
    Null,
    Bool(bool),
    Number(SsztNumber),
    String(String),
    Vector(Box<Vec<SsztValue>>),
    List(Box<Vec<SsztValue>>),
    Container(IndexMap<String, Box<SsztValue>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SsztNumber {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(U256),
}

impl Prefixable for SsztNumber {
    fn prefixed() -> bool {
        false
    }
}

impl Encode for SsztNumber {
    fn encode_to<W: Output>(&self, dest: &mut W) {
        match self {
            SsztNumber::U8(n) => [*n].encode_to(dest),
            SsztNumber::U16(n) => n.encode_to(dest),
            SsztNumber::U32(n) => n.encode_to(dest),
            SsztNumber::U64(n) => n.encode_to(dest),
            SsztNumber::U128(n) => n.encode_to(dest),
            SsztNumber::U256(n) => {
                let mut result = [0u8; 32];
                for i in 0..32 {
                    result[i] = n.byte(i);
                }

                result.encode_to(dest)
            }
        }
    }
}
