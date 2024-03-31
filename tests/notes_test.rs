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
    fn test_add_note() {
        let mut entity = MyEntity::new();
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
    }

    #[test]
    fn test_delete_note() {
        let mut entity = MyEntity::new();
        let _ = entity.insert_note(
            "fsmith".to_string(),
            "This was updated".as_bytes().to_vec(),
            None,
        );
        let id = entity.insert_note(
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

        entity.remove_note(id);

        assert_eq!(entity.notes.len(), 2);
    }

    #[test]
    fn test_get_note_ok() {
        let mut entity = MyEntity::new();
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
        
        assert_eq!(entity.get_note(id).unwrap().content_as_string().unwrap(), "This was updated".to_string());
    }

    #[test]
    fn test_get_note_bad() {
        let entity = MyEntity::new();

        match entity.get_note("1234".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }

    #[test]
    fn test_search_notes() {
        let mut entity = MyEntity::new();
        let _ = entity.insert_note(
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

        let search_results = entity.search_notes("thing".to_string());
        assert_eq!(search_results.len(), 2);
    }

    #[test]
    fn test_special_characters() {
        let mut entity = MyEntity::new();
        let msg = "Th帝s is a speciàl character messagæ.";
        let id = entity.insert_note(
            "someone".to_string(),
            msg.as_bytes().to_vec(),
            None,
        );
        
        assert_eq!(entity.get_note(id).unwrap().content_as_string().unwrap(), msg.to_string());
    }

    #[test]
    fn test_update_note() {
        let mut entity = MyEntity::new();
        let id = entity.insert_note(
            "fsmith".to_string(),
            "This was updated".as_bytes().to_vec(),
            None,
        );

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
    }

}
