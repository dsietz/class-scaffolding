// #[macro_use]
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_macros::*;
    use std::collections::BTreeMap;

    #[scaffolding_struct("metadata")]
    #[derive(Debug, Clone)]
    struct MyEntity {
        a: String,
        b: bool,
    }

    impl MyEntity {
        #[scaffolding_fn("metadata")]
        pub fn new(param1: String, param2: bool) -> Self {
            let x = format!("a_{}", param1);
            Self {
                id: "lorem ipsum".to_string(),
                created_dtm: 1711281509,
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
            metadata: BTreeMap::new(),
            a: "hello".to_string(),
            b: true,
        };

        assert_eq!(entity.id, "lorem ipsum");
        assert_eq!(entity.b, true);
        assert_eq!(entity.hello(), "hello");
    }

    #[test]
    fn test_core_impl() {
        let mut entity = MyEntity::new("hello".to_string(), true);

        assert_eq!(entity.id, "lorem ipsum");
        assert_eq!(entity.b, true);
        assert_eq!(entity.hello(), "a_hello");
    }
}
