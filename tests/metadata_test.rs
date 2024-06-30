extern crate scaffolding_core;

#[cfg(test)]
mod tests {
    use scaffolding_core::*;

    #[scaffolding_struct("metadata")]
    #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding)]
    struct MyEntity {}

    impl MyEntity {
        #[scaffolding_fn("metadata")]
        fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn test_entity_new() {
        let mut entity = MyEntity::new();

        // scaffolding metadata
        entity
            .metadata
            .insert("field_1".to_string(), "myvalue".to_string());
        assert_eq!(entity.metadata.len(), 1);
    }
}
