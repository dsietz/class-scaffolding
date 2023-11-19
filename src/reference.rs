#[macro_use]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAPISourceDefinition {
    document: String,
}

impl OpenAPISourceDefinition {
    pub fn new(document: String) -> Self {
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
    // use serde_json::*;
    use std::fs;

    fn read_openapi_json () -> String {
        fs::read_to_string("./tests/openapi-petstore.json").unwrap()
    }

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
        let api = OpenAPISourceDefinition::new(read_openapi_json());
        assert_eq!(api.document, read_openapi_json());
    }

    #[test]
    fn test_openapi_srcdef_deserialize() {
        let serialized = format!("{{\"document\": \"{}\"}}", read_openapi_json());
        println!("{}", serialized);
        // let api = OpenAPISourceDefinition::from_serialized(&serialized);

        // assert_eq!(api.document, serialized);
    }

    #[test]
    fn test_openapi_srcdef_serialize() {
        let mut api = OpenAPISourceDefinition::new(read_openapi_json());
        
        assert_eq!(api.serialize(), read_openapi_json());
    }

    // #[test]
    // fn test_() {
    //     let data = fs::read_to_string("./tests/openapi-petstore.json").unwrap();
    //     let j: Value = serde_json::from_str(&data).unwrap();
    //     let m = j.as_object().unwrap();

    //     println!("{:?}", m.get("info").unwrap());
    // }
}
