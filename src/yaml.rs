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

        assert_eq!(to_ssz(json.as_bytes().into()), [4, 0, 0, 0, 0, 1, 2, 3]);
    }

    #[test]
    fn variable_size() {
        let json = r#"
            "lists":
                "a_list":
                    - "list"
                    - "0:u8"
                    - "1:u8"
                    - "2:u8"
                    - "3:u8"
            "#;

        assert_eq!(
            to_ssz(json.as_bytes().into()),
            [12, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0, 0, 1, 2, 3]
        );
    }
}
