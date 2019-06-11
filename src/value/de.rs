use crate::value::{SsztNumber, SsztValue};
use ethereum_types::U256;
use indexmap::IndexMap;
use serde::de::{self, Deserialize, Deserializer, Visitor};

struct SszValueVisitor;

impl<'de> Visitor<'de> for SszValueVisitor {
    type Value = SsztValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid sszt")
    }

    fn visit_none<E>(self) -> Result<SsztValue, E> {
        Ok(SsztValue::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<SsztValue, E>
    where
        E: de::Error,
    {
        match value {
            true => Ok(SsztValue::Bool(true)),
            false => Ok(SsztValue::Bool(false)),
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<SsztValue, E>
    where
        E: de::Error,
    {
        let parsed: Vec<&str> = value.split(":").collect();

        if parsed.len() == 2 {
            let n = parsed[0];
            let t = parsed[1];

            match t {
                "u8" => Ok(SsztValue::Number(SsztNumber::U8(n.parse::<u8>().unwrap()))),
                "u16" => Ok(SsztValue::Number(SsztNumber::U16(
                    n.parse::<u16>().unwrap(),
                ))),
                "u32" => Ok(SsztValue::Number(SsztNumber::U32(
                    n.parse::<u32>().unwrap(),
                ))),
                "u64" => Ok(SsztValue::Number(SsztNumber::U64(
                    n.parse::<u64>().unwrap(),
                ))),
                "u128" => Ok(SsztValue::Number(SsztNumber::U128(
                    n.parse::<u128>().unwrap(),
                ))),
                "u256" => Ok(SsztValue::Number(SsztNumber::U256(
                    U256::from_dec_str(&n[..]).unwrap(),
                ))),
                t => panic!("Unknown integer type: {}", t),
            }
        } else {
            Ok(SsztValue::String(value.into()))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<SsztValue, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        if let Some(value) = seq.next_element::<String>()? {
            let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(value) = seq.next_element::<SsztValue>()? {
                values.push(value);
            }

            match value.as_str() {
                "vector" => Ok(SsztValue::Vector(Box::new(values))),
                "list" => Ok(SsztValue::List(Box::new(values))),
                v => panic!("Unknown array type: {:?}", v),
            }
        } else {
            unimplemented!()
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<SsztValue, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut object = IndexMap::new();
        while let Some(value) = map.next_entry::<String, SsztValue>()? {
            object.insert(value.0, Box::new(value.1));
        }

        Ok(SsztValue::Container(object))
    }
}

impl<'de> Deserialize<'de> for SsztValue {
    fn deserialize<D>(deserializer: D) -> Result<SsztValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SszValueVisitor)
    }
}
