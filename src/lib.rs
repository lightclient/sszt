use serde_json::Value;
use ssz::Encode;

pub fn json_to_ssz(data: Vec<u8>) -> Vec<u8> {
    let v: Value = serde_json::from_slice(&data).unwrap();
    serialize(&v)
}

fn serialize(v: &Value) -> Vec<u8> {
    match v {
        Value::Null => vec![],
        Value::Bool(b) => b.encode(),
        Value::Number(_) => unreachable!(),
        Value::String(s) => {
            let parsed: Vec<&str> = s.split(":").collect();

            if parsed.len() == 2 {
                let n = parsed[0];
                let t = parsed[1];

                match t {
                    "u8" => vec![n.parse::<u8>().unwrap()],
                    "u16" => n.parse::<u16>().unwrap().encode(),
                    "u32" => n.parse::<u32>().unwrap().encode(),
                    "u64" => n.parse::<u64>().unwrap().encode(),
                    "u128" => n.parse::<u128>().unwrap().encode(),
                    t => panic!("Unknown integer type: {}", t),
                }
            } else {
                s.encode()
            }
        }
        Value::Array(a) => {
            if let Value::String(t) = &a[0] {
                match &t[..] {
                    "list" => {
                        let list = a[1..]
                            .iter()
                            .fold(vec![], |acc, v| [&acc[..], &serialize(v)[..]].concat());

                        let mut len = (list.len() as u32).encode();
                        len.extend(list);

                        len
                    }
                    "vector" => a[1..]
                        .iter()
                        .fold(vec![], |acc, v| [&acc[..], &serialize(v)[..]].concat()),
                    t => panic!("Array should define it's type; got {}", t),
                }
            } else {
                panic!("Array should define it's type");
            }
        }
        Value::Object(o) => {
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

fn is_fixed_length(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::Bool(_) => true,
        Value::Number(_) => true,
        Value::String(_) => true,
        Value::Array(a) => {
            if let Value::String(t) = &a[0] {
                match &t[..] {
                    "list" => false,
                    "vector" => true,
                    _ => panic!("Array should define it's type"),
                }
            } else {
                panic!("Array should define it's type");
            }
        }
        Value::Object(o) => o.values().map(|v| is_fixed_length(v)).any(|x| x == true),
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
