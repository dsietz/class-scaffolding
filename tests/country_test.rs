extern crate scaffolding_core;

#[cfg(test)]
mod tests {
    use scaffolding_core::*;

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
}
