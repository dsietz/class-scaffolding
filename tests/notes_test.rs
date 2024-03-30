extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_core::{defaults, ActivityItem, Note, Scaffolding, ScaffoldingNotes};
    use scaffolding_macros::*;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[scaffolding_struct("notes")]
    #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingNotes)]
    struct MyEntity {}

    impl MyEntity {
        #[scaffolding_fn("notes")]
        fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn test_entity_new() {
        let mut entity = MyEntity::new();

        // scaffolding notes
        let id = entity.insert_note(
            "fsmith".to_string(),
            "This was updated".as_bytes().to_vec(),
            None,
        );
        assert_eq!(entity.notes.len(), 1);

        entity.modify_note(
            id.clone(),
            "fsmith".to_string(),
            "This was updated again".as_bytes().to_vec(),
            Some("private".to_string()),
        );

        assert_eq!(
            entity.get_note(id.clone()).unwrap().access,
            "private".to_string()
        );
        assert_eq!(
            entity.get_note(id.clone()).unwrap().content,
            "This was updated again".as_bytes().to_vec()
        );
        assert_eq!(entity.notes.len(), 1);
    }
}
