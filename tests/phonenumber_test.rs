extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use scaffolding_core::*;
    use scaffolding_macros::*;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[scaffolding_struct("phone_numbers")]
    #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingPhoneNumbers)]
    struct MyEntity {}

    impl MyEntity {
        #[scaffolding_fn("phone_numbers")]
        fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn test_phone_new() {
        let phone = PhoneNumber::new(
            "home".to_string(),
            "8482493561".to_string(),
            "USA".to_string(),
        );
        let now = Utc::now().timestamp();

        // scaffolding attributes
        assert_eq!(phone.id.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
        assert_eq!(phone.created_dtm, now);
        assert_eq!(phone.modified_dtm, now);
    }

    #[test]
    fn test_countries_get_phone() {
        let countries = Countries::new();

        match countries.get_country_by_phone_code("1".to_string()) {
            Some(country) => {
                assert_eq!(country.name, "United States");
                assert_eq!(country.phone_code, "1");
                assert_eq!(country.iso_2_code, "US");
                assert_eq!(country.iso_3_code, "USA");
            }
            None => assert!(false),
        }
    }

    #[test]
    fn test_entity_phonenumbers() {
        let mut entity = MyEntity::new();

        assert_eq!(entity.phone_numbers.len(), 0);

        let phone1 = entity.insert_phone_number(
            "home".to_string(),
            "8482493561".to_string(),
            "USA".to_string(),
        );

        let phone2 = entity.insert_phone_number(
            "work".to_string(),
            "2223330000".to_string(),
            "USA".to_string(),
        );

        let _ = entity.insert_phone_number(
            "other".to_string(),
            "7776664444".to_string(),
            "USA".to_string(),
        );

        assert_eq!(phone1.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
        assert_eq!(entity.phone_numbers.len(), 3);

        entity.remove_phone_number(phone2);
        assert_eq!(entity.phone_numbers.len(), 2);

        let home = entity.search_phone_numbers_by_category("home".to_string());
        assert_eq!(home.len(), 1);
        assert_eq!(home[0].category, "home".to_string());
    }
}
