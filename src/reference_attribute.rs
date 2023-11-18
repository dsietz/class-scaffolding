extern crate uuid;

use http::Request;
use std::fmt;
// use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use uuid::Uuid;

#[derive(Clone, Debug)]
enum ReferenceAttribute {
    ASSIGNED(String),
    GENERATED(),
    HTTP(http::Request<()>),
}

impl fmt::Display for ReferenceAttribute {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
        // write!(f, "{:?}", self)
    }
}

impl ReferenceAttribute {
    pub fn new(v: Option<String>) -> Self {
        match v {
            Some(s) => {
                return ReferenceAttribute::ASSIGNED(s);
            }
            None => {
                return ReferenceAttribute::ASSIGNED(Uuid::new_v4().to_string());
            }
        }
    }

    pub fn as_value(&self) -> String {
        match self {
            Self::ASSIGNED(s) => {
                return s.to_string();
            }
            Self::GENERATED() => {
                return Uuid::new_v4().to_string();
            }
            Self::HTTP(r) => {
                let (parts, body) = r.clone().into_parts();
                let mut hdrs = Vec::new();

                for key in parts.headers.keys() {
                    let hdr = json!({key.to_string(): parts.headers[key].to_str().unwrap()});
                    hdrs.push(hdr);
                }

                let obj = json!({
                    "uri": parts.uri.to_string(),
                    "method": parts.method.to_string(),
                    "headers": hdrs,
                    "body": body
                });

                return obj.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ReferenceAttribute, Request};

    #[test]
    fn test_reference_attribute_new_some() {
        let expected: String = String::from("CUST1324657890");
        let attr: ReferenceAttribute = ReferenceAttribute::new(Some(expected.clone()));
        // println!("{}", attr.as_value());
        assert_eq!(attr.as_value(), expected);
    }

    #[test]
    fn test_reference_attribute_new_none() {
        let attr: ReferenceAttribute = ReferenceAttribute::new(None);
        let expected: String = attr.as_value();
        // println!("{}", attr.as_value());
        assert_eq!(attr.as_value(), expected);
    }

    #[test]
    fn test_reference_attribute_generated() {
        let expected: String = ReferenceAttribute::GENERATED().as_value();
        // println!("{}, {}", expected.clone(), ReferenceAttribute::GENERATED().as_value());
        assert_ne!(ReferenceAttribute::GENERATED().as_value(), expected);
    }

    #[test]
    fn test_reference_attribute_http() {
        let request = Request::builder()
            .method("GET")
            .uri("https://www.rust-lang.org/")
            .header("X-Custom-Foo", "Bar")
            .body(())
            .unwrap();
        let expected = r#"{"body":null,"headers":[{"x-custom-foo":"Bar"}],"method":"GET","uri":"https://www.rust-lang.org/"}"#;
        assert_eq!(ReferenceAttribute::HTTP(request).as_value(), expected);
    }
}
