extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use scaffolding_core::*;
    use scaffolding_macros::*;
    use serde_derive::{Deserialize, Serialize};

    #[scaffolding_struct("addresses")]
    #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingAddresses)]
    struct MyEntity {}

    impl MyEntity {
        #[scaffolding_fn("addresses")]
        fn new() -> Self {
            Self {}
        }
    }

    #[test]
    fn test_address_new() {
        let address = Address::new(
            "shipping".to_string(),
            "acmes company".to_string(),
            "14 Main Street".to_string(),
            "Big City, NY 038845".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );
        let now = Utc::now().timestamp();

        // scaffolding attributes
        assert_eq!(
            address.id.len(),
            "54324f57-9e6b-4142-b68d-1d4c86572d0a".len()
        );
        assert_eq!(address.created_dtm, now);
        assert_eq!(address.modified_dtm, now);
    }

    #[test]
    fn test_countries() {
        let countries = Countries::new();

        assert_eq!(countries.list.len(), 240);
        assert_eq!(countries.list[0].name, "Afghanistan");
        assert_eq!(countries.list[0].phone_code, "93");
        assert_eq!(countries.list[0].iso_2_code, "AF");
        assert_eq!(countries.list[0].iso_3_code, "AFG");
    }

    #[test]
    fn test_countries_get_iso_2() {
        let countries = Countries::new();

        match countries.get_country_by_iso_2_code("US".to_string()) {
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
    fn test_countries_get_iso_3() {
        let countries = Countries::new();

        match countries.get_country_by_iso_3_code("USA".to_string()) {
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
    fn test_countries_is_valid_true() {
        let countries = Countries::new();
        let country = Country::new(
            "United States".to_string(),
            "1".to_string(),
            "US".to_string(),
            "USA".to_string(),
        );

        assert_eq!(countries.is_valid(country), true);
    }

    #[test]
    fn test_countries_is_valid_false() {
        let countries = Countries::new();
        let country = Country::new(
            "United States".to_string(),
            "1".to_string(),
            "US".to_string(),
            "ABC".to_string(),
        );

        assert_eq!(countries.is_valid(country), false);
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
    fn test_country_new() {
        let country = Country::new(
            "United States".to_string(),
            "1".to_string(),
            "US".to_string(),
            "USA".to_string(),
        );

        assert_eq!(country.name, "United States".to_string());
        assert_eq!(country.phone_code, "1".to_string());
        assert_eq!(country.iso_2_code, "US".to_string());
        assert_eq!(country.iso_3_code, "USA".to_string());
    }

    #[test]
    fn test_entity_addresses() {
        let mut entity = MyEntity::new();

        assert_eq!(entity.addresses.len(), 0);

        let addr1 = entity.add_address(
            "shipping".to_string(),
            "acmes company".to_string(),
            "14 Main Street".to_string(),
            "Big City, NY 038845".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        let addr2 = entity.add_address(
            "billing".to_string(),
            "acmes company".to_string(),
            "14 Main Street".to_string(),
            "Big City, NY 038845".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        let addr3 = entity.add_address(
            "home".to_string(),
            "Peter Petty".to_string(),
            "23 Corner Lane".to_string(),
            "Tiny Town, VT 044567".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        let addr4 = entity.add_address(
            "shipping".to_string(),
            "neighbor house".to_string(),
            "24 Corner Lane".to_string(),
            "Tiny Town, VT 044567".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        assert_eq!(addr1.id.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
        assert_eq!(entity.addresses.len(), 4);

        entity.remove_address(addr2.id);
        assert_eq!(entity.addresses.len(), 3);

        let shipping = entity.addresses_by_category("shipping".to_string());
        assert_eq!(shipping.len(), 2);
        assert_eq!(shipping[0].category, "shipping".to_string());
        assert_eq!(shipping[1].category, "shipping".to_string());
    }
}
