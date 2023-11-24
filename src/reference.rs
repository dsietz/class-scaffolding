use openapiv3::OpenAPI;
use serde_json::{json, Value};

const DATA: &'static str = "data";
const ENGINE: &'static str = "engine";
const EXTERNAL: &'static str = "external";
const HTTP: &'static str = "http";
const HTTPS: &'static str = "https";
const INTERNAL: &'static str = "internal";

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct ReferenceEngine {}

impl ReferenceEngine {
    pub fn get_reference_data() -> Value {
        json!("hello")
    }
}

//Source
// OpenAPI must be v3
#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum ReferenceSource {
    API(OpenAPI),
}

impl ReferenceSource {
    pub fn from_serialized(serialized: &str) -> Self {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// Value
#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum ReferenceValue {
    NUMBER(usize),
    TEXT(String),
}

impl ReferenceValue {
    pub fn to_string(&self) -> String {
        match self {
            Self::TEXT(s) => s.to_string(),
            Self::NUMBER(n) => n.to_string(),
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::TEXT(s) => s.to_string().parse::<usize>().unwrap(),
            Self::NUMBER(n) => *n,
        }
    }

    pub fn from_serialized(serialized: &str) -> Self {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httptest::{matchers::*, responders::*, Expectation, Server};
    use serde_json::json;
    use serde_yaml::value::Value;

    #[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
    struct contact {
        first_name: String,
        last_name: String,
        email: String,
        phone: String,
        company: String,
        department: String,
    }

    fn get_contact_by_id(id: usize) -> contact {
        let raw = include_str!("../tests/contact_data.yaml");
        let data: Value = serde_yaml::from_str(raw).unwrap();
        let filtered: Vec<_> = data
            .as_sequence()
            .unwrap()
            .iter()
            .filter(|item| item.as_mapping().unwrap().get("id").unwrap() == id)
            .map(|item| {
                let c: contact = serde_yaml::from_value(item.clone()).unwrap();
                c
            })
            .collect();

        filtered[0].clone()
    }

    fn mock_service() -> Server {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/customer/27"))
                .respond_with(json_encoded(json!(get_contact_by_id(27)))),
        );

        println!("Service listening at: {}", server.addr());

        server
    }

    #[tokio::test]
    async fn test_get_ref_data() {
        // init the test service
        let server = mock_service();
        let url = server.url("/customer/27");

        // perform the api call
        let body = reqwest::get(url.to_string())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        assert_eq!(
            body,
            r#"{"company":"Lazz","department":"Marketing","email":"npooleyq@digg.com","first_name":"Noam","last_name":"Pooley","phone":"205-147-6793"}"#
        );
    }

    #[test]
    fn test_refsrc_serialization() {
        let data = include_str!("../tests/openapi.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let mut orig_src = ReferenceSource::API(openapi);
        let new_src = ReferenceSource::from_serialized(&orig_src.serialize());

        assert_eq!(orig_src, new_src);
    }

    #[test]
    fn test_refval_from_serialized() {
        let val = ReferenceValue::from_serialized(r#"{"TEXT":"123456789"}"#);

        assert_eq!(val.to_string(), "123456789".to_string());
        assert_eq!(val.to_usize(), 123456789);
    }

    #[test]
    fn test_refval_serialize_string() {
        let mut val = ReferenceValue::TEXT("123456789".to_string());

        assert_eq!(val.serialize(), r#"{"TEXT":"123456789"}"#);
    }

    #[test]
    fn test_refval_serialize_usize() {
        let mut val = ReferenceValue::NUMBER(123456789);

        assert_eq!(val.serialize(), r#"{"NUMBER":123456789}"#);
    }

    #[test]
    fn test_refval_string_to_string() {
        let val = ReferenceValue::TEXT("123456789".to_string());

        assert_eq!(val.to_string(), "123456789".to_string());
    }

    #[test]
    fn test_refval_string_to_usize() {
        let val = ReferenceValue::TEXT("123456789".to_string());

        assert_eq!(val.to_usize(), 123456789);
    }

    #[test]
    fn test_refval_usize_to_string() {
        let val = ReferenceValue::NUMBER(123456789);

        assert_eq!(val.to_string(), "123456789".to_string());
    }

    #[test]
    fn test_refval_usize_to_usize() {
        let val = ReferenceValue::NUMBER(123456789);

        assert_eq!(val.to_usize(), 123456789);
    }
}
