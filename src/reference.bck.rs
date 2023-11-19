extern crate uuid;

use http::{Error, Request};
use serde::{Serialize, Serializer};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug)]
enum ReferenceValue {
    ASSIGNED(String),
    HTTP(http::Request<()>),
    NUMBER(usize),
    UUID(Option<String>),
}

impl Serialize for ReferenceValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Display for ReferenceValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceValue::ASSIGNED(s) => {
                write!(f, "{}", s)
            }
            ReferenceValue::HTTP(r) => {
                write!(f, "{}", r.uri().to_string())
            }
            ReferenceValue::NUMBER(n) => {
                write!(f, "{}", n.to_string())
            }
            ReferenceValue::UUID(u) => match u {
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

impl ReferenceValue {
    pub fn new(v: Option<String>) -> Self {
        match v {
            Some(s) => {
                return ReferenceValue::ASSIGNED(s);
            }
            None => {
                return ReferenceValue::UUID(Some(Uuid::new_v4().to_string()));
            }
        }
    }

    pub fn as_request(&self) -> Result<Request<()>, http::Error> {
        Request::builder()
            .method("GET")
            .uri(self.to_string())
            .body(())
    }

    pub fn as_usize(&self) -> Result<usize, core::num::ParseIntError> {
        self.to_string().parse::<usize>()
    }

    pub fn serialize(&mut self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Obj {
        id: ReferenceValue,
        is_employee: bool,
        salary: usize,
    }

    #[test]
    fn test_identitier_attribute_as_request() {
        let expected = Request::builder()
            .method("GET")
            .uri("https://www.rust-lang.org/")
            .body(())
            .unwrap();
        let uri: ReferenceValue = ReferenceValue::HTTP(expected.clone());

        assert_eq!(uri.as_request().unwrap().uri(), expected.uri());
    }

    #[test]
    fn test_identitier_attribute_as_usize() {
        let expected: usize = 1234567890;
        let number: ReferenceValue = ReferenceValue::NUMBER(expected);

        assert_eq!(number.as_usize().unwrap(), expected);
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

        assert_eq!(ReferenceValue::HTTP(request).to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_new_some() {
        let expected: String = String::from("CUST1324657890");
        let attr: ReferenceValue = ReferenceValue::new(Some(expected.clone()));

        assert_eq!(attr.to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_new_none() {
        let attr: ReferenceValue = ReferenceValue::new(None);
        let expected: String = attr.to_string();

        assert_eq!(attr.to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_number() {
        let expected = r#"1234567890"#;

        assert_eq!(ReferenceValue::NUMBER(1234567890).to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_scenario() {
        let record_uuid_none: Obj = Obj {
            id: ReferenceValue::UUID(None),
            is_employee: true,
            salary: 150000,
        };
        let record_uuuid_some: Obj = Obj {
            id: ReferenceValue::UUID(Some("b98e54a0-d105-4379-a1f7-54351ddbdbd3".to_string())),
            is_employee: true,
            salary: 150000,
        };
        let request = Request::builder()
            .method("GET")
            .uri("https://www.rust-lang.org/")
            .header("X-Custom-Foo", "Bar")
            .body(())
            .unwrap();
        let record_http: Obj = Obj {
            id: ReferenceValue::HTTP(request),
            is_employee: true,
            salary: 150000,
        };
        let record_number: Obj = Obj {
            id: ReferenceValue::NUMBER(1234567890),
            is_employee: true,
            salary: 150000,
        };

        assert_eq!(record_uuid_none.id.to_string().len(), 36);
        assert_eq!(
            record_uuuid_some.id.to_string(),
            "b98e54a0-d105-4379-a1f7-54351ddbdbd3".to_string()
        );
        assert_eq!(
            record_http.id.to_string(),
            "https://www.rust-lang.org/".to_string()
        );
        assert_eq!(record_number.id.to_string(), "1234567890".to_string());
    }

    #[test]
    fn test_identitier_attribute_uuid_none() {
        let id: ReferenceValue = ReferenceValue::UUID(None);
        let expected: String = id.to_string();

        assert_ne!(id.to_string(), expected);
    }

    #[test]
    fn test_identitier_attribute_uuid_some() {
        let expected: String = "b98e54a0-d105-4379-a1f7-54351ddbdbd3".to_string();
        let id = ReferenceValue::UUID(Some(expected.clone()));

        assert_eq!(id.to_string(), expected);
    }
}
