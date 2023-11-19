extern crate uuid;

// use super::*;
use http::Request;
use std::fmt;
use serde::{Serialize, Serializer};
// use serde_derive::Serialize;
// use serde::ser::{Serialize, SerializeStruct, Serializer};
use uuid::Uuid;

#[derive(Clone, Debug)]
enum IdentitierValue {
    ASSIGNED(String),
    HTTP(http::Request<()>),
    NUMBER(usize),
    UUID(Option<String>),
}

impl Serialize for IdentitierValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Display for IdentitierValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentitierValue::ASSIGNED(s) => {
                write!(f, "{}", s)
            }
            IdentitierValue::HTTP(r) => {
                write!(f, "{}", r.uri().to_string())
            }
            IdentitierValue::NUMBER(n) => {
                write!(f, "{}", n.to_string())
            }
            IdentitierValue::UUID(u) => match u {
                Some(i) => {
                    write!(f, "{}", i.to_string())
                }
                None => {
                    write!(f, "{}", Uuid::new_v4().to_string())
                }
            },
        }
    }
}

impl IdentitierValue {
    pub fn new(v: Option<String>) -> Self {
        match v {
            Some(s) => {
                return IdentitierValue::ASSIGNED(s);
            }
            None => {
                return IdentitierValue::UUID(Some(Uuid::new_v4().to_string()));
            }
        }
    }

    pub fn serialize(&mut self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;
    use serde_derive::Serialize;

    #[derive(Debug, Serialize)]
    struct Obj {
        id: IdentitierValue,
        is_employee: bool,
        salary: usize,
    }

    impl Obj {
        pub fn new(identifier: IdentitierValue, employeed: bool, pay: usize) -> Self {
            Self {
                id: identifier,
                is_employee: employeed,
                salary: pay
            }
        }

        pub fn serialize(&mut self) -> String {
            serde_json::to_string(&self).unwrap()
        }
    }

    #[test]
    fn test_identitier_attribute_new_some() {
        let expected: String = String::from("CUST1324657890");
        let attr: IdentitierValue = IdentitierValue::new(Some(expected.clone()));

        assert_eq!(attr.to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_new_none() {
        let attr: IdentitierValue = IdentitierValue::new(None);
        let expected: String = attr.to_string();

        assert_eq!(attr.to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_uuid_none() {
        let id: IdentitierValue = IdentitierValue::UUID(None);
        let expected: String = id.to_string();

        assert_ne!(id.to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_uuid_some() {
        let expected: String = "b98e54a0-d105-4379-a1f7-54351ddbdbd3".to_string();
        let id = IdentitierValue::UUID(Some(expected.clone()));

        assert_eq!(id.to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_http() {
        let request = Request::builder()
            .method("GET")
            .uri("https://www.rust-lang.org/")
            .header("X-Custom-Foo", "Bar")
            .body(())
            .unwrap();
        let expected = r#"https://www.rust-lang.org/"#;
        assert_eq!(IdentitierValue::HTTP(request).to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_number() {
        let expected = r#"1234567890"#;
        assert_eq!(
            IdentitierValue::NUMBER(1234567890).to_string(),
            expected
        );
    }

    #[test]
    fn test_scenario() {
        let mut record_one: Obj = Obj::new(IdentitierValue::UUID(None),true,150000);

        println!("{}", record_one.serialize());
    }
}
