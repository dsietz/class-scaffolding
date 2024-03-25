// #[macro_use]
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_macros::*;
    use std::collections::BTreeMap;

    #[scaffolding_entity("metadata")]
    #[derive(Debug, Clone)]
    struct MyEntity {
        b: bool,
    }

    #[test]
    fn test_core_struct() {
        let entity = MyEntity {
            id: "lorem ipsum".to_string(),
            created_dtm: 1711281509,
            modified_dtm: 1711281509,
            inactive_dtm: 1711281509,
            expired_dtm: 1711281509,
            metadata: BTreeMap::new(),
            b: true,
        };
        println!("struct is {:?}", entity);

        assert_eq!(entity.id, "lorem ipsum");
        assert_eq!(entity.b, true);
    }
}
