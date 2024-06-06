extern crate scaffolding_macros;

use super::{defaults, ActivityItem, Scaffolding};
use scaffolding_macros::*;
use serde_json::{Result, Value};

/// Supporting Classes
pub struct Countries {
    pub list: Vec<Country>,
}

impl Countries {
    /// This is the constructor function.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::entity::Countries;
    ///
    /// fn main() {
    ///   let countries = Countries::new();
    /// }
    /// ```
    pub fn new() -> Self {
        let data = include_str!("countries.json");
        let array: Value = serde_json::from_str(data).unwrap();
        let countries: Vec<Country> = array
            .as_array()
            .unwrap()
            .iter()
            .map(|c| {
                Country::new(
                    c["country_name"].as_str().unwrap().to_string(),
                    c["phone_code"].as_str().unwrap().to_string(),
                    c["iso_2_code"].as_str().unwrap().to_string(),
                    c["iso_3_code"].as_str().unwrap().to_string(),
                )
            })
            .collect();
        Self { list: countries }
    }

    /// Verifies a Country
    ///
    /// ### Example
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::entity::{Countries, Country};
    ///
    /// fn main() {
    ///  let countries = Countries::new();
    ///   let country = Country::new(
    ///       "United States".to_string(),
    ///       "1".to_string(),
    ///       "US".to_string(),
    ///       "USA".to_string()
    ///   );
    ///
    ///   assert_eq!(countries.is_valid(country), true);
    /// }
    /// ```
    pub fn is_valid(&self, country: Country) -> bool {
        let found = self.list.iter().filter(|c| {
            c.name == country.name
                && c.phone_code == country.phone_code
                && c.iso_2_code == country.iso_2_code
                && c.iso_3_code == country.iso_3_code
        });
        match found.count() {
            0 => return false,
            _ => return true,
        }
    }

    /// Retrieves a Country based on the ISO 2 Code
    ///
    /// ### Example
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::entity::{Countries, Country};
    ///
    /// fn main() {
    ///  let countries = Countries::new();
    ///  let country = countries.get_country_by_iso_2_code("US".to_string()).unwrap();
    ///
    ///  assert_eq!(country.name, "United States");
    ///  assert_eq!(country.phone_code, "1");
    ///  assert_eq!(country.iso_2_code, "US");
    ///  assert_eq!(country.iso_3_code, "USA");
    /// }
    /// ```
    pub fn get_country_by_iso_2_code(&self, iso_2_code: String) -> Option<&Country> {
        let found = self.list.iter().filter(|c| c.iso_2_code == iso_2_code);

        return found.last();
    }

    /// Retrieves a Country based on the ISO 3 Code
    ///
    /// ### Example
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::entity::{Countries, Country};
    ///
    /// fn main() {
    ///  let countries = Countries::new();
    ///  let country = countries.get_country_by_iso_3_code("USA".to_string()).unwrap();
    ///
    ///  assert_eq!(country.name, "United States");
    ///  assert_eq!(country.phone_code, "1");
    ///  assert_eq!(country.iso_2_code, "US");
    ///  assert_eq!(country.iso_3_code, "USA");
    /// }
    /// ```
    pub fn get_country_by_iso_3_code(&self, iso_3_code: String) -> Option<&Country> {
        let found = self.list.iter().filter(|c| c.iso_3_code == iso_3_code);

        return found.last();
    }

    /// Retrieves a Country based on the international phone code
    ///
    /// ### Example
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::entity::{Countries, Country};
    ///
    /// fn main() {
    ///  let countries = Countries::new();
    ///  let country = countries.get_country_by_phone_code("1".to_string()).unwrap();
    ///
    ///  assert_eq!(country.name, "United States");
    ///  assert_eq!(country.phone_code, "1");
    ///  assert_eq!(country.iso_2_code, "US");
    ///  assert_eq!(country.iso_3_code, "USA");
    /// }
    /// ```
    pub fn get_country_by_phone_code(&self, phone_code: String) -> Option<&Country> {
        let found = self.list.iter().filter(|c| c.phone_code == phone_code);

        return found.last();
    }
}

// A country definition
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Country {
    // Textual name of the coutnry
    pub name: String,
    // The code used for international phone calls
    pub phone_code: String,
    // The 2 char abbreviation
    pub iso_2_code: String,
    // The 3 char abbreviation
    pub iso_3_code: String,
}

impl Country {
    /// This is the constructor function.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::entity::Country;
    ///
    /// fn main() {
    ///    let country = Country::new("United States".to_string(), "1".to_string(), "US".to_string(), "USA".to_string());
    ///
    ///    assert_eq!(country.name, "United States".to_string());
    ///    assert_eq!(country.phone_code, "1".to_string());
    ///    assert_eq!(country.iso_2_code, "US".to_string());
    ///    assert_eq!(country.iso_3_code, "USA".to_string());
    /// }
    /// ```
    pub fn new(name: String, phone_code: String, iso_2_code: String, iso_3_code: String) -> Self {
        Self {
            name: name,
            phone_code: phone_code,
            iso_2_code: iso_2_code,
            iso_3_code: iso_3_code,
        }
    }
}

#[scaffolding_struct]
#[derive(Clone, Debug, Deserialize, Serialize, Scaffolding)]
pub struct Address {
    // The type of address, (e.g.: Billing, Shipping, Home, Work, etc.)
    pub category: String,
    // The first line of the address should contain the location's full name
    pub line_1: String,
    // The second line of the address should include the house number and street address/ PO box address
    pub line_2: String,
    // The third line of the address should include the city name followed by province, state, or county name and postal code
    pub line_3: String,
    // The fourth line of the address including the country
    pub line_4: String,
    // The country code of the location (Use Alpha 3 codes)
    pub country_code: String,
}

impl Address {
    /// This is the constructor function.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::entity::Address;
    ///
    /// fn main() {
    ///   let entity = Address::new(
    ///       "shipping".to_string(),
    ///       "acmes company".to_string(),
    ///       "14 Main Street".to_string(),
    ///       "Big City, NY 038845".to_string(),
    ///       "USA".to_string(),
    ///       "USA".to_string(),
    ///   
    ///   );
    ///   
    ///   // scaffolding attributes
    ///   println!("{}", entity.id);
    ///   println!("{}", entity.created_dtm);
    ///   println!("{}", entity.modified_dtm,);
    ///   println!("{}", entity.inactive_dtm);
    ///   println!("{}", entity.expired_dtm );
    /// }
    /// ```
    #[scaffolding_fn]
    pub fn new(
        category: String,
        line_1: String,
        line_2: String,
        line_3: String,
        line_4: String,
        country_code: String,
    ) -> Self {
        Self {
            category: category,
            line_1: line_1,
            line_2: line_2,
            line_3: line_3,
            line_4: line_4,
            country_code: country_code,
        }
    }
}

#[scaffolding_struct]
#[derive(Clone, Debug, Deserialize, Serialize, Scaffolding)]
pub struct Entity {
    // The type of address, (e.g.: Billing, Shipping, Home, Work, etc.)
    pub category: String,
}

impl Entity {
    /// This is the constructor function.
    ///
    /// #Example
    ///
    /// ```rust
    /// // extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::{defaults, ActivityItem};
    ///
    /// let mut activity_item = ActivityItem::new("updated".to_string(), "This was updated".to_string());
    /// ```
    #[scaffolding_fn]
    pub fn new(category: String) -> Self {
        Self { category: category }
    }
}
