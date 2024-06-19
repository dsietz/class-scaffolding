extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use scaffolding_core::*;
    use scaffolding_macros::*;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use std::fs;

    #[scaffolding_struct("addresses", "email_addresses", "metadata", "notes", "phone_numbers", "tags")]
    #[derive(
        Clone,
        Debug,
        Deserialize,
        Serialize,
        Scaffolding,
        ScaffoldingAddresses,
        ScaffoldingEmailAddresses,
        ScaffoldingNotes,
        ScaffoldingPhoneNumbers,
        ScaffoldingTags,
    )]
    struct MyEntity {
        b: bool,
        n: i64,
    }

    impl MyEntity {
        #[scaffolding_fn("addresses", "email_addresses", "metadata", "notes", "phone_numbers", "tags")]
        fn new(arg: bool) -> Self {
            Self {
                b: arg,
                n: defaults::never(),
            }
        }

        fn my_func(&self) -> String {
            format!("The answer is {}", self.b)
        }
    }

    fn get_entity() -> MyEntity {
        let mut entity = MyEntity::new(true);

        // activity logs
        entity.log_activity(
            "updated".to_string(),
            "The object has been updated".to_string(),
        );
        entity.log_activity(
            "updated".to_string(),
            "The object has been updated".to_string(),
        );
        entity.log_activity(
            "cancelled".to_string(),
            "The object has been cancelled".to_string(),
        );

        // addresses
        let _ = entity.insert_address(
            "shipping".to_string(),
            "acmes company".to_string(),
            "14 Main Street".to_string(),
            "Big City, NY 038845".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        let _ = entity.insert_address(
            "billing".to_string(),
            "acmes company".to_string(),
            "14 Main Street".to_string(),
            "Big City, NY 038845".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        let _ = entity.insert_address(
            "home".to_string(),
            "Peter Petty".to_string(),
            "23 Corner Lane".to_string(),
            "Tiny Town, VT 044567".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        let _ = entity.insert_address(
            "shipping".to_string(),
            "neighbor house".to_string(),
            "24 Corner Lane".to_string(),
            "Tiny Town, VT 044567".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        // email addresses
        let _ = entity.insert_email_address("home".to_string(), "myemail@example.com".to_string());
        let _ = entity.insert_email_address("work".to_string(), "myemail@example.com".to_string());
        let _ = entity.insert_email_address("other".to_string(), "myemail@example.com".to_string());

        // metadata
        entity
            .metadata
            .insert("field_1".to_string(), "myvalue1".to_string());
        entity
            .metadata
            .insert("field_2".to_string(), "myvalue2".to_string());

        // notes
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

        // phone numbers
        let _ = entity.insert_phone_number(
            "home".to_string(),
            "8482493561".to_string(),
            "USA".to_string(),
        );

        let _ = entity.insert_phone_number(
            "work".to_string(),
            "2223330000".to_string(),
            "USA".to_string(),
        );

        let _ = entity.insert_phone_number(
            "other".to_string(),
            "7776664444".to_string(),
            "USA".to_string(),
        );

        // tags
        entity.add_tag("tag_1".to_string());
        entity.add_tag("tag_2".to_string());
        entity.add_tag("tag_3".to_string());

        entity
    }

    #[test]
    fn test_entity_all() {
        let entity = get_entity();

        // activity logs
        assert_eq!(entity.activity.len(), 3);
        assert_eq!(entity.get_activity("updated".to_string()).len(), 2);

        // addresses
        assert_eq!(entity.addresses.len(), 4);
        assert_eq!(
            entity
                .search_addresses_by_category("shipping".to_string())
                .len(),
            2
        );

        // email addresses
        assert_eq!(entity.email_addresses.len(), 3);

        // metadata
        assert_eq!(entity.metadata.len(), 2);

        // notes
        assert_eq!(entity.notes.len(), 3);

        // phone numbers
        assert_eq!(entity.phone_numbers.len(), 3);

        // tags
        assert_eq!(entity.tags.len(), 3);
    }

    #[test]
    fn test_entity_all_serialization() {
        let mut json = fs::read_to_string("./tests/entity.json").expect("Cannot read the entity.json file");
        json.retain(|c| !c.is_whitespace());
        let mut entity = MyEntity::deserialized::<MyEntity>(json.as_bytes()).unwrap();


        assert_eq!(entity.my_func(), "The answer is true".to_string());
        assert_eq!(entity.get_activity("updated".to_string()).len(), 2);
        
        assert_eq!(entity.addresses.len(), 4);
        assert_eq!(entity.search_addresses_by_category("shipping".to_string()).len(), 2);

        assert_eq!(entity.email_addresses.len(), 3);
        assert_eq!(entity.search_email_addresses_by_category("home".to_string()).len(), 1);

        assert_eq!(entity.metadata.len(), 2);

        assert_eq!(entity.notes.len(), 3);
        assert_eq!(entity.search_notes("thing".to_string()).len(), 2);

        assert_eq!(entity.phone_numbers.len(), 3);
        assert_eq!(entity.search_phone_numbers_by_category("home".to_string()).len(), 1);

        assert_eq!(entity.tags.len(), 3);
        assert!(entity.has_tag("tag_1".to_string()));

        assert_eq!(entity.serialize(), json);
    }
}
