
#[macro_use]
use serde_json::*;
use utoipa::*;

const DATA: &'static str = "data";
const ENGINE: &'static str = "engine";
const EXTERNAL: &'static str = "external";
const HTTP: &'static str = "http";
const HTTPS: &'static str = "https";
const INTERNAL: &'static str = "internal";

pub struct DatabaseSourceDefinition {
    engine: String
} 

pub struct OpenAPISourceDefinition {
    document: String
}

pub enum SourceDefinition { 
    DatabaseSourceDefinition,
    OpenAPISourceDefinition
}

pub struct ReferenceSource {
    // The unique identifier of the source
    id: String,
    // Indicates where to reference: EXTERNAL or INTERNAL
    scope: String,
    // The type of reference: DATA or ENGINE
    ref_type: String,
    // The definition of the source: DatabaseSourceDefinition or OpenAPISourceDefinition
    definition: SourceDefinition
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_() {
        let data = fs::read_to_string("./tests/openapi-petstore.json").unwrap();
        let j: Value = serde_json::from_str(&data).unwrap();
        let m = j.as_object().unwrap();

        println!("{:?}", m.get("info").unwrap());
    }
}
