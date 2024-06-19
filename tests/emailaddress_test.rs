extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use scaffolding_core::*;
    use scaffolding_macros::*;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[scaffolding_struct("email_addresses")]
    #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingEmailAddresses)]
    struct MyEntity {}

    impl MyEntity {
        #[scaffolding_fn("email_addresses")]
        fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn test_email_new() {
        let email = EmailAddress::new("home".to_string(), "myemail@example.com".to_string());
        let now = Utc::now().timestamp();

        // scaffolding attributes
        assert_eq!(email.id.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
        assert_eq!(email.created_dtm, now);
        assert_eq!(email.modified_dtm, now);
    }

    #[test]
    fn test_email_is_valid_true() {
        let email = EmailAddress::new("home".to_string(), "myemail@example.com".to_string());

        assert_eq!(email.is_valid(), true);
    }

    #[test]
    fn test_email_is_valid_false() {
        let email = EmailAddress::new("home".to_string(), "myemail@example".to_string());

        assert_eq!(email.is_valid(), false);
    }

    #[test]
    fn test_entity_emailaddresses() {
        let mut entity = MyEntity::new();

        assert_eq!(entity.email_addresses.len(), 0);

        let email1 =
            entity.insert_email_address("home".to_string(), "myemail@example.com".to_string());

        let email2 =
            entity.insert_email_address("work".to_string(), "myemail@example.com".to_string());

        let _ = entity.insert_email_address("other".to_string(), "myemail@example.com".to_string());

        assert_eq!(email1.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
        assert_eq!(entity.email_addresses.len(), 3);

        entity.remove_email_address(email2);
        assert_eq!(entity.email_addresses.len(), 2);

        let home = entity.search_email_addresses_by_category("home".to_string());
        assert_eq!(home.len(), 1);
        assert_eq!(home[0].category, "home".to_string());
    }
}
