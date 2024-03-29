// #[macro_use]
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_macros::*;
    use std::collections::BTreeMap;
    use chrono::{DateTime, Duration, Months, Utc};
    use uuid::Uuid;

    // #[scaffolding_struct("metadata")]
    #[scaffolding_struct]
    #[derive(Debug, Clone)]
    struct MyEntity {
        a: String,
        b: bool,
    }

    // #[access_methods(((system_constant, mod_par.system_constants, PyValueType),))]
    // #[access_methods(((unit, units, PyUnit),))]
    // #[access_methods(((group, groups, PyGroup),))]
    // #[pymethods]
    // #[scaffolding_impl((|a,b|{}))]
    impl MyEntity {
        #[scaffolding_fn]
        pub fn new(param1: String, param2: bool) -> Self {
            let x = format!("a_{}", param1);
            Self {
                // id: "lorem ipsum".to_string(),
                // created_dtm: 1711281509,
                modified_dtm: 1711281509,
                inactive_dtm: 1711281509,
                expired_dtm: 1711281509,
                a: x,
                b: param2,
            }
        }

        #[scaffolding_fn]
        pub fn hello(&self) -> String {
            self.a.clone()
        }
    }

    #[test]
    fn test_core_struct() {
        let mut entity = MyEntity {
            id: "lorem ipsum".to_string(),
            created_dtm: 1711281509,
            modified_dtm: 1711281509,
            inactive_dtm: 1711281509,
            expired_dtm: 1711281509,
            // metadata: BTreeMap::new(),
            a: "hello".to_string(),
            b: true,
        };
        // println!("struct is {:?}", entity);

        // assert_eq!(entity.id, "lorem ipsum");
        assert_eq!(entity.b, true);
        assert_eq!(entity.hello(), "hello");

        // entity.set_id("too bad".to_string());
        // entity.id = "too bad".to_string();
        // assert_eq!(entity.get_id(), "too bad");
        // assert_eq!(entity.get_a(), "hello");
    }

    #[test]
    fn test_core_impl() {
        let mut entity = MyEntity::new("hello".to_string(), true);
        // println!("struct is {:?}", entity);

        assert_eq!(entity.id, "unique id");
        assert_eq!(entity.b, true);
        assert_eq!(entity.hello(), "a_hello");

        // entity.set_id("too bad".to_string());
        // entity.id = "too bad".to_string();
        // assert_eq!(entity.get_id(), "too bad");
        // assert_eq!(entity.get_a(), "hello");
    }
}
