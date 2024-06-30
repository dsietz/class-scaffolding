extern crate scaffolding_core;

#[cfg(test)]
mod tests {
    use scaffolding_core::*;

    #[scaffolding_struct("tags")]
    #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingTags)]
    struct MyEntity {}

    impl MyEntity {
        #[scaffolding_fn("tags")]
        fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn test_entity_new() {
        let mut entity = MyEntity::new();

        // scaffolding tags
        entity.add_tag("tag_1".to_string());
        // ignore any duplicates
        entity.add_tag("tag_1".to_string());
        entity.add_tag("tag_2".to_string());
        entity.add_tag("tag_3".to_string());

        assert_eq!(entity.tags.len(), 3);
        assert!(entity.has_tag("tag_1".to_string()));

        entity.remove_tag("tag_2".to_string());

        assert_eq!(entity.tags.len(), 2);
        assert!(entity.has_tag("tag_1".to_string()));
        assert!(entity.has_tag("tag_3".to_string()));
    }
}
