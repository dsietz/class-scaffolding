// #[macro_use]
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_macros::*;

    #[scaffolding_entity(METADATA)]
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
            b: true,
        };
        println!("struct is {:?}", entity);

        assert_eq!(entity.id, "lorem ipsum");
        assert_eq!(entity.b, true);
    }
}
