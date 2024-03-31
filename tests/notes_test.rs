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
        let _ = entity.insert_note(
            "fsmith".to_string(),
            "Something to find here".as_bytes().to_vec(),
            None,
        );
        let _ = entity.insert_note(
            "fsmith".to_string(),
            "Nonething to find here".as_bytes().to_vec(),
            Some("private".to_string()),
        );
        assert_eq!(entity.notes.len(), 3);

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
            entity
                .get_note(id.clone())
                .unwrap()
                .content_as_string()
                .unwrap(),
            "This was updated again".to_string()
        );
        assert_eq!(entity.notes.len(), 3);

        let search_results = entity.search_notes("thing".to_string());
        assert_eq!(search_results.len(), 2);

        entity.remove_note(id);
        assert_eq!(entity.notes.len(), 2);
    }
}
