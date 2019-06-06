use indexmap::IndexMap;
use serde::Deserialize;
use ssz::Encode;

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
enum SszType {
    Fixed(SszFixedType),
    Variable(SszVariableType),
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
enum SszVariableType {
    List(Box<SszType>),
    // Union(Box<SszType>),
    Container(Box<SszType>),
    Vector(Box<SszType>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value")]
enum SszFixedType {
    Container(Box<IndexMap<String, SszFixedType>>),
    Vector(Box<Vec<SszFixedType>>),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    // Uint256(u256),
    Bool(bool),
}

pub fn json_to_ssz(data: Vec<u8>) -> Vec<u8> {
    let v: SszFixedType = serde_json::from_slice(&data).unwrap();
    dfs_json(&v)
}

fn dfs_json(def: &SszFixedType) -> Vec<u8> {
    match def {
        SszFixedType::Uint8(i) => vec![*i],
        SszFixedType::Uint16(i) => i.encode(),
        SszFixedType::Uint32(i) => i.encode(),
        SszFixedType::Uint64(i) => i.encode(),
        SszFixedType::Uint128(i) => i.encode(),
        SszFixedType::Bool(b) => b.encode(),
        SszFixedType::Container(c) => c
            .values()
            .fold(vec![], |acc, v| [&acc[..], &dfs_json(v)[..]].concat()),
        SszFixedType::Vector(v) => v
            .iter()
            .fold(vec![], |acc, v| [&acc[..], &dfs_json(v)[..]].concat()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size() {
        let json = r#"     
{
    "type": "Container",
    "value": {
        "fixed": {
            "type": "Vector",
            "value": [
                {
                    "type": "Uint8",
                    "value": 0
                },
                {
                    "type": "Uint8",
                    "value": 1
                },
                {
                    "type": "Uint8",
                    "value": 2
                },
                {
                    "type": "Uint8",
                    "value": 3
                },
                {
                    "type": "Uint8",
                    "value": 4
                }
            ]
        },
        "other": {
            "type": "Container",
            "value": {
                "a": {
                    "type": "Uint16",
                    "value": 16
                },
                "b": {
                    "type": "Uint32",
                    "value": 32
                }
            }
        }
    }
}
"#;

        let f: SszFixedType = serde_json::from_str(json).unwrap();
        assert_eq!(dfs_json(&f), [0, 1, 2, 3, 4, 16, 0, 32, 0, 0, 0]);
    }
}
