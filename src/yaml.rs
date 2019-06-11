use crate::ser::serialize;
use crate::value::SsztValue;
use serde_yaml;

pub fn to_ssz(data: Vec<u8>) -> Vec<u8> {
    let v: SsztValue = serde_yaml::from_slice(&data).unwrap();
    serialize(&v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use ssz::{Decode, Encode};

    #[test]
    fn simple_fixed_size() {
        let yaml = r#"
        fixed:
            - vector
            - 0:u8
            - 1:u8
            - 2:u8
            - 3:u8
            - 4:u8
        other:
            a: 16:u16
            b: 32:u32
        "#;

        assert_eq!(
            to_ssz(yaml.as_bytes().into()),
            [0, 1, 2, 3, 4, 16, 0, 32, 0, 0, 0]
        );
    }

    #[test]
    fn recursive_fixed_size() {
        let json = r#"
        fixed:
            - vector
            -
                - vector
                - 0:u8
            -
                - vector
                - 1:u8
            -
                - vector
                - 2:u8
            -
                - vector
                - 3:u8
            -
                - vector
                - 4:u8
        other:
            a:
                b:
                    c:
                        d:
                            - vector
                            - 16:u16
            b: 32:u32
        "#;

        assert_eq!(
            to_ssz(json.as_bytes().into()),
            [0, 1, 2, 3, 4, 16, 0, 32, 0, 0, 0]
        );
    }

    #[test]
    fn simple_variable_size() {
        let json = r#"
            - "list"
            - "0:u8"
            - "1:u8"
            - "2:u8"
            - "3:u8"
        "#;

        assert_eq!(
            to_ssz(json.as_bytes().into()),
            vec![0u8, 1u8, 2u8, 3u8].encode()
        );
    }

    #[derive(Debug, PartialEq, Ssz)]
    struct A {
        a: B,
    }

    #[derive(Debug, PartialEq, Ssz)]
    struct B {
        b: Vec<u8>,
    }

    #[test]
    fn variable_size() {
        let a = A {
            a: B {
                b: vec![0, 1, 2, 3],
            },
        };

        let json = r#"
            "lists":
                "a_list":
                    - "list"
                    - "0:u8"
                    - "1:u8"
                    - "2:u8"
                    - "3:u8"
            "#;

        assert_eq!(to_ssz(json.as_bytes().into()), a.encode());
    }

    #[test]
    fn empty_container_list() {
        #[derive(Debug, PartialEq, Ssz)]
        struct A {
            a: B,
        }

        #[derive(Debug, PartialEq, Ssz)]
        struct B {
            b: Vec<B>,
        }

        let a = A { a: B { b: vec![] } };

        let yaml = r#"
            a:
                b:
                    - list
        "#;

        let encoded = to_ssz(yaml.as_bytes().into());
        assert_eq!(encoded, a.encode());

        let decoded = A::decode(&mut &encoded[..]).unwrap();
        assert_eq!(decoded, a);
    }

    #[derive(Debug, PartialEq, Ssz, Default)]
    struct Message {
        pub timestamp: u64,
        pub message: [u8; 32],
    }

    #[derive(Debug, PartialEq, Ssz, Default)]
    struct State {
        pub messages: Vec<Message>,
    }

    #[derive(Debug, PartialEq, Ssz, Default)]
    struct InputBlock {
        pub new_messages: Vec<Message>,
        pub state: State,
    }

    #[test]
    fn bazaar_messages() {
        let block = InputBlock {
            new_messages: vec![
                Message {
                    timestamp: 1,
                    message: [0u8; 32],
                },
                Message {
                    timestamp: 2,
                    message: [1u8; 32],
                },
            ],
            state: State { messages: vec![] },
        };

        let yaml = r#"
        new_messages:
                - list
                - timestamp: 1:u64
                  message: 0:u256
                - timestamp: 2:u64
                  message: 454086624460063511464984254936031011189294057512315937409637584344757371137:u256
        state:
            messages:
                - list
        "#;

        let encoded = to_ssz(yaml.as_bytes().into());
        assert_eq!(encoded, block.encode());

        let decoded = InputBlock::decode(&mut &encoded[..]).unwrap();
        assert_eq!(decoded, block);
    }

    #[test]
    fn bazaar_state() {
        let state = State { messages: vec![] };

        let yaml = r#"
        messages:
            - list
        "#;

        let encoded = to_ssz(yaml.as_bytes().into());
        assert_eq!(encoded, state.encode());

        let decoded = State::decode(&mut &encoded[..]).unwrap();
        assert_eq!(decoded, state);
    }
}
