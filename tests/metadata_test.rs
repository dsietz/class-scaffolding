extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    use scaffolding_macros::*;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::BTreeMap;

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
