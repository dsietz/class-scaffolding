#[macro_use]
use openapiv3::OpenAPI;
use uuid::Uuid;

const DATA: &'static str = "data";
const ENGINE: &'static str = "engine";
const EXTERNAL: &'static str = "external";
const HTTP: &'static str = "http";
const HTTPS: &'static str = "https";
const INTERNAL: &'static str = "internal";

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseSourceDefinition {
    engine: String,
}

impl DatabaseSourceDefinition {
    pub fn new(engine: String) -> Self {
        Self { engine: engine }
    }

    pub fn from_serialized(serialized: &str) -> Self {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OpenAPISourceDefinition {
    document: OpenAPI,
}

impl OpenAPISourceDefinition {
    pub fn new(document: OpenAPI) -> Self {
        Self { document: document }
    }

    pub fn from_serialized(serialized: &str) -> Self {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SourceDefinition {
    DatabaseSourceDefinition,
    OpenAPISourceDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceSource {
    // The unique identifier of the source
    id: String,
    // Indicates where to reference: EXTERNAL or INTERNAL
    scope: String,
    // The type of reference: DATA or ENGINE
    ref_type: String,
    // The definition of the source: DatabaseSourceDefinition or OpenAPISourceDefinition
    definition: SourceDefinition,
}

impl ReferenceSource {
    pub fn new(
        identifier: Option<String>,
        scope: String,
        ref_type: String,
        definition: SourceDefinition,
    ) -> Self {
        let uid = match identifier {
            Some(i) => i,
            None => Uuid::new_v4().to_string(),
        };

        Self {
            id: uid,
            scope: scope,
            ref_type: ref_type,
            definition: definition,
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

    #[test]
    fn test_db_srcdef_new() {
        let db = DatabaseSourceDefinition::new("dynamodb".to_string());
        assert_eq!(db.engine, "dynamodb".to_string());
    }

    #[test]
    fn test_db_srcdef_deserialize() {
        let serialized = r#"{"engine": "dynamodb"}"#;
        let db = DatabaseSourceDefinition::from_serialized(&serialized);

        assert_eq!(db.engine, "dynamodb".to_string());
    }

    #[test]
    fn test_db_srcdef_serialize() {
        let expected = r#"{"engine":"dynamodb"}"#;
        let mut db = DatabaseSourceDefinition::new("dynamodb".to_string());
        
        assert_eq!(db.serialize(), expected);
    }

    #[test]
    fn test_openapi_srcdef_new() {
        let data = include_str!("../tests/openapi-petstore.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());

        assert_eq!(api.document, openapi);
    }

    #[test]
    fn test_openapi_srcdef_serialize_deserialize() {
        let data = include_str!("../tests/openapi-petstore.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let mut orig_api: OpenAPISourceDefinition = OpenAPISourceDefinition::new(openapi);
        let serialized: String = orig_api.serialize();
        let new_api: OpenAPISourceDefinition = OpenAPISourceDefinition::from_serialized(&serialized);
        
        assert_eq!(orig_api, new_api);
    }
}
