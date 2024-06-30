extern crate scaffolding_core;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use scaffolding_core::*;

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
    fn test_entity_addresses() {
        let mut entity = MyEntity::new();

        assert_eq!(entity.addresses.len(), 0);

        let addr1 = entity.insert_address(
            "shipping".to_string(),
            "acmes company".to_string(),
            "14 Main Street".to_string(),
            "Big City, NY 038845".to_string(),
            "USA".to_string(),
            "USA".to_string(),
        );

        let addr2 = entity.insert_address(
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

        assert_eq!(addr1.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
        assert_eq!(entity.addresses.len(), 4);

        assert_eq!(entity.get_address(addr1.clone()).unwrap().id, addr1);

        entity.remove_address(addr2);
        assert_eq!(entity.addresses.len(), 3);

        let shipping = entity.search_addresses_by_category("shipping".to_string());
        println!("{:?}", shipping);
        assert_eq!(shipping.len(), 2);
        assert_eq!(shipping[0].category, "shipping".to_string());
        assert_eq!(shipping[1].category, "shipping".to_string());
    }
}
