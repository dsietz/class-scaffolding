#[macro_use]
use openapiv3::OpenAPI;
use uuid::Uuid;

const DATA: &'static str = "data";
const ENGINE: &'static str = "engine";
const EXTERNAL: &'static str = "external";
const HTTP: &'static str = "http";
const HTTPS: &'static str = "https";
const INTERNAL: &'static str = "internal";

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum SourceDefinition {
    DB(DatabaseSourceDefinition),
    API(OpenAPISourceDefinition),
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum ReferenceSourceScope {
    EXTERNAL,
    INTERNAL
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum ReferenceSourceType {
    DATA,
    ENGINE
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct ReferenceSource {
    // The unique identifier of the source
    identifier: String,
    // Indicates where to reference: EXTERNAL or INTERNAL
    scope: ReferenceSourceScope,
    // The type of reference: DATA or ENGINE
    ref_type: ReferenceSourceType,
    // The definition of the source: DatabaseSourceDefinition or OpenAPISourceDefinition
    definition: SourceDefinition,
}

impl ReferenceSource {
    pub fn new(
        identifier: Option<String>,
        scope: ReferenceSourceScope,
        ref_type: ReferenceSourceType,
        definition: SourceDefinition,
    ) -> Self {
        let uid = match identifier {
            Some(i) => i,
            None => Uuid::new_v4().to_string(),
        };

        Self {
            identifier: uid,
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
    fn test_refsrc_new_api_with_id() {
        let data = include_str!("../tests/openapi-petstore.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());

        let src = ReferenceSource::new(
            Some("4cf2add2-11f3-450b-9f2f-fe4035c82161".to_string()),
            ReferenceSourceScope::EXTERNAL,
            ReferenceSourceType::DATA,
            SourceDefinition::API(api.clone())
        );

        assert_eq!(src.identifier, "4cf2add2-11f3-450b-9f2f-fe4035c82161".to_string());
        assert_eq!(src.scope, ReferenceSourceScope::EXTERNAL);
        assert_eq!(src.ref_type, ReferenceSourceType::DATA);
        assert_eq!(src.definition, SourceDefinition::API(api));
    }

    #[test]
    fn test_refsrc_new_api_without_id() {
        let data = include_str!("../tests/openapi-petstore.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());

        let src = ReferenceSource::new(
            None,
            ReferenceSourceScope::EXTERNAL,
            ReferenceSourceType::DATA,
            SourceDefinition::API(api.clone())
        );

        assert_eq!(src.identifier.len(), 36);
        assert_eq!(src.scope, ReferenceSourceScope::EXTERNAL);
        assert_eq!(src.ref_type, ReferenceSourceType::DATA);
        assert_eq!(src.definition, SourceDefinition::API(api));
    }

    #[test]
    fn test_refsrc_serialize_deserialize_api() {
        let data = include_str!("../tests/openapi-petstore.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());
        let mut orig_src = ReferenceSource::new(
            None,
            ReferenceSourceScope::EXTERNAL,
            ReferenceSourceType::DATA,
            SourceDefinition::API(api.clone())
        );
        let serialized = orig_src.serialize();
        let new_src = ReferenceSource::from_serialized(&serialized);

        assert_eq!(orig_src, new_src);
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
