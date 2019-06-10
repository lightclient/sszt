use crate::value::SsztValue;
use ssz::Encode;

pub fn serialize(v: &SsztValue) -> Vec<u8> {
    match v {
        SsztValue::Null => vec![],
        SsztValue::Bool(b) => b.encode(),
        SsztValue::Number(n) => n.encode(),
        SsztValue::String(s) => s.encode(),
        SsztValue::Vector(v) => v
            .iter()
            .fold(vec![], |acc, val| [&acc[..], &serialize(val)[..]].concat()),
        SsztValue::List(l) => {
            let list = l
                .iter()
                .fold(vec![], |acc, val| [&acc[..], &serialize(val)[..]].concat());

            let mut len = (l.len() as u32).encode();
            len.extend(list);

            len
        }
        SsztValue::Container(o) => {
            if is_fixed_length(v) {
                o.values()
                    .fold(vec![], |acc, v| [&acc[..], &serialize(v)[..]].concat())
            } else {
                let list = o
                    .values()
                    .fold(vec![], |acc, v| [&acc[..], &serialize(v)[..]].concat());

                let mut len = (list.len() as u32).encode();
                len.extend(list);

                len
            }
        }
    }
}

fn is_fixed_length(v: &SsztValue) -> bool {
    match v {
        SsztValue::Null => true,
        SsztValue::Bool(_) => true,
        SsztValue::Number(_) => true,
        SsztValue::String(_) => true,
        SsztValue::Vector(_) => true,
        SsztValue::List(_) => false,
        SsztValue::Container(o) => o.values().map(|v| is_fixed_length(v)).any(|x| x == true),
    }
}
