#[macro_use]
extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_core::{defaults, ActivityItem};
    use scaffolding_macros::*;
    use std::collections::BTreeMap;

    #[scaffolding_struct("metadata")]
    #[derive(Debug, Clone)]
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
