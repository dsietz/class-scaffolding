#[macro_use]
use std::fmt;
use either::*;
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
    INTERNAL,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum ReferenceSourceType {
    DATA,
    ENGINE,
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

// Object

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum ReferenceValueDataType {
    NUMBER(usize),
    TEXT(String),
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct ReferenceValue {
    // The path to use when retrieving the value, (e.g.: /customer/{customerId})
    path: String,
    // The parameter to use in the path, (e.g.: customerId)
    param: ReferenceValueDataType,
}

impl ReferenceValue {
    pub fn new(path: String, param: ReferenceValueDataType) -> Self {
        Self {
            path: path,
            param: param,
        }
    }

    pub fn from_serialized(serialized: &str) -> Self {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum ReferenceObjectValue {
    REFERENCE(ReferenceValue),
    NUMBER(usize),
    TEXT(String),
    UUID(Option<String>),
}

impl fmt::Display for ReferenceObjectValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceObjectValue::REFERENCE(r) => {
                let rv = match r.param.clone() {
                    ReferenceValueDataType::NUMBER(n) => {
                        n.to_string()
                    },
                    ReferenceValueDataType::TEXT(s) => {
                        s
                    },
                };

                write!(f, "{}", rv)
            }
            ReferenceObjectValue::NUMBER(n) => {
                write!(f, "{}", n.to_string())
            }
            ReferenceObjectValue::TEXT(s) => {
                write!(f, "{}", s)
            }
            ReferenceObjectValue::UUID(u) => match u {
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

impl ReferenceObjectValue {
    pub fn new(v: Option<String>) -> Self {
        match v {
            Some(s) => {
                return ReferenceObjectValue::TEXT(s);
            }
            None => {
                return ReferenceObjectValue::UUID(Some(Uuid::new_v4().to_string()));
            }
        }
    }

    // pub fn as_usize(&self) -> Result<usize, core::num::ParseIntError> {
    //     self.to_string().parse::<usize>()
    // }

    pub fn from_serialized(serialized: &str) -> Self {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub struct ReferenceObject {
    key: String,
    value: Option<ReferenceObjectValue>,
    source_identifier: String,
}

impl ReferenceObject {
    pub fn new(
        key: Option<String>,
        value: Option<ReferenceObjectValue>,
        source_identifier: String,
    ) -> Self {
        let k = match key {
            Some(s) => {
                s
            },
            None => {
                Uuid::new_v4().to_string()
            }
        };

        // Handle generating Uuid for ReferenceObjectValue::UUID(None) 
        let v = match value {
            Some(val) => {
                match val {
                    ReferenceObjectValue::UUID(u) => {
                        match u {
                            Some(id) => {
                                Some(ReferenceObjectValue::UUID(Some(id)))
                            },
                            None => {
                                Some(ReferenceObjectValue::UUID(Some(Uuid::new_v4().to_string())))
                            }
                        }
                    },
                    _ => {
                        Some(val)
                    }
                }
            },
            None => {
                value
            }
        };

        Self {
            key: k,
            value: v,
            source_identifier: source_identifier,
        }
    }

    pub fn from_serialized(serialized: &str) -> Self {
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn value(&self) -> Either<String, usize> {
        match self.value.clone() {
            Some(v) => {
                match v {
                    ReferenceObjectValue::REFERENCE(r) => {
                        match r.param {
                            ReferenceValueDataType::TEXT(s) => {
                                Left(s)
                            },
                            ReferenceValueDataType::NUMBER(n) => {
                                Right(n)
                            },
                        }
                    },
                    ReferenceObjectValue::NUMBER(n) => {
                        Right(n)
                    },
                    ReferenceObjectValue::TEXT(s) => {
                        Left(s)
                    },
                    ReferenceObjectValue::UUID(u) => {
                        match u {
                            Some(s) => {
                                Left(s)
                            },
                            None => {
                                Left("".to_string())
                            }
                        }
                        
                    },
                }
            },
            None => {
                Right(0)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refobj_new_number() {
        let refsrc_id = "4cf2add2-11f3-450b-9f2f-fe4035c82161".to_string();
        let refval = 1234567980;
        let refobj = ReferenceObject::new(
            None,
            Some(ReferenceObjectValue::NUMBER(refval)),
            refsrc_id.clone()
        );
        let val = refobj.value();

        assert_eq!(refobj.key.len(), 36);
        assert_eq!(refobj.source_identifier, refsrc_id);
        assert_eq!(val.is_left(), false);
        assert_eq!(val.is_right(), true);
        assert_eq!(refobj.value().right().unwrap(), refval);
    }

    #[test]
    fn test_refobj_new_text() {
        let refsrc_id = "4cf2add2-11f3-450b-9f2f-fe4035c82161".to_string();
        let refval = "C1234567890".to_string();
        let refobj = ReferenceObject::new(
            None,
            Some(ReferenceObjectValue::TEXT(refval.clone())),
            refsrc_id.clone()
        );
        let val = refobj.value();

        assert_eq!(refobj.key.len(), 36);
        assert_eq!(refobj.source_identifier, refsrc_id);
        assert_eq!(val.is_left(), true);
        assert_eq!(val.is_right(), false);
        assert_eq!(refobj.value().left().unwrap(), refval);
    }

    #[test]
    fn test_refobj_new_text_empty() {
        let refsrc_id = "4cf2add2-11f3-450b-9f2f-fe4035c82161".to_string();
        let refobj = ReferenceObject::new(
            None,
            Some(ReferenceObjectValue::UUID(None)),
            refsrc_id.clone()
        );
        let val = refobj.value();

        assert_eq!(refobj.key.len(), 36);
        assert_eq!(refobj.source_identifier, refsrc_id);
        assert_eq!(val.is_left(), true);
        assert_eq!(val.is_right(), false);
        assert_eq!(refobj.value().left().unwrap().len(), 36);
    }

    #[test]
    fn test_refsrc_new_api_with_id() {
        let data = include_str!("../tests/openapi.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());

        let src = ReferenceSource::new(
            Some("4cf2add2-11f3-450b-9f2f-fe4035c82161".to_string()),
            ReferenceSourceScope::EXTERNAL,
            ReferenceSourceType::DATA,
            SourceDefinition::API(api.clone()),
        );

        assert_eq!(
            src.identifier,
            "4cf2add2-11f3-450b-9f2f-fe4035c82161".to_string()
        );
        assert_eq!(src.scope, ReferenceSourceScope::EXTERNAL);
        assert_eq!(src.ref_type, ReferenceSourceType::DATA);
        assert_eq!(src.definition, SourceDefinition::API(api));
    }

    #[test]
    fn test_refsrc_new_api_without_id() {
        let data = include_str!("../tests/openapi.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());

        let src = ReferenceSource::new(
            None,
            ReferenceSourceScope::EXTERNAL,
            ReferenceSourceType::DATA,
            SourceDefinition::API(api.clone()),
        );

        assert_eq!(src.identifier.len(), 36);
        assert_eq!(src.scope, ReferenceSourceScope::EXTERNAL);
        assert_eq!(src.ref_type, ReferenceSourceType::DATA);
        assert_eq!(src.definition, SourceDefinition::API(api));
    }

    #[test]
    fn test_refsrc_serialize_deserialize_api() {
        let data = include_str!("../tests/openapi.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());
        let mut orig_src = ReferenceSource::new(
            None,
            ReferenceSourceScope::EXTERNAL,
            ReferenceSourceType::DATA,
            SourceDefinition::API(api.clone()),
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
        let data = include_str!("../tests/openapi.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let api = OpenAPISourceDefinition::new(openapi.clone());

        assert_eq!(api.document, openapi);
    }

    #[test]
    fn test_openapi_srcdef_serialize_deserialize() {
        let data = include_str!("../tests/openapi.json");
        let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
        let mut orig_api: OpenAPISourceDefinition = OpenAPISourceDefinition::new(openapi);
        let serialized: String = orig_api.serialize();
        let new_api: OpenAPISourceDefinition =
            OpenAPISourceDefinition::from_serialized(&serialized);

        assert_eq!(orig_api, new_api);
    }
}
