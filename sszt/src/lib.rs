mod value;

use ssz::Encode;
use value::{SsztNumber, SsztValue};

pub fn json_to_ssz(data: Vec<u8>) -> Vec<u8> {
    let v: SsztValue = serde_json::from_slice(&data).unwrap();
    serialize(&v)
}

fn serialize(v: &SsztValue) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_fixed_size() {
        let json = r#"{   
            "fixed": [
                "vector",
                "0:u8",
                "1:u8",
                "2:u8",
                "3:u8",
                "4:u8"
            ],
            "other": {
                "a": "16:u16",
                "b": "32:u32"
            }
        }"#;

        assert_eq!(
            json_to_ssz(json.as_bytes().into()),
            [0, 1, 2, 3, 4, 16, 0, 32, 0, 0, 0]
        );
    }

    #[test]
    fn test_recursive_fixed_size() {
        let json = r#"{   
            "fixed": [
                "vector",
                ["vector", "0:u8"],
                ["vector", "1:u8"],
                ["vector", "2:u8"],
                ["vector", "3:u8"],
                ["vector", "4:u8"]
            ],
            "other": {
                "a": {
                    "b": {
                        "c": {
                            "d": [
                                "vector",
                                "16:u16"
                            ]
                        }
                    }
                },
                "b": "32:u32"
            }
        }"#;

        assert_eq!(
            json_to_ssz(json.as_bytes().into()),
            [0, 1, 2, 3, 4, 16, 0, 32, 0, 0, 0]
        );
    }

    #[test]
    fn test_simple_variable_size() {
        let json = r#"[
            "list",
            "0:u8",
            "1:u8",
            "2:u8",
            "3:u8"
        ]"#;

        assert_eq!(
            json_to_ssz(json.as_bytes().into()),
            [4, 0, 0, 0, 0, 1, 2, 3]
        );
    }

    #[test]
    fn test_variable_size() {
        let json = r#"{
            "lists": {
                "a_list": [
                    "list",
                    "0:u8",
                    "1:u8",
                    "2:u8",
                    "3:u8"
                ]
            }
        }"#;

        assert_eq!(
            json_to_ssz(json.as_bytes().into()),
            [12, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0, 0, 1, 2, 3]
        );
    }
}
