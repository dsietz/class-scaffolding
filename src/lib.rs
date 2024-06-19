//! Object-oriented programming (OOP) has been around since the 1960s and was first introduced in the late 1950s
//! in artificial intelligence by an MMIT group. It is no wonder then that over the years, the concept of objects
//! being represented by classes and attributes with inheritanted behavior.
//!
//! Rust addresses this design by providing structures, traits, and implementations. However, the native ability to
//! `extend` a class (like in other languages) makes OOP a bit of a challenge. To address this gap, `Scaffolding` utilizes
//! Rust's [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) to mimic the ability to  
//! `extend` a class - both data structure and behavior.
//!
//! ## Scaffolding Concept
//! 1. A class that `extends` the "Scaffolding class" should inherate all the "parent" data structure and behavior,
//!    as well as append the "child" specific data structure and behavior from the generic type being extended.
//! 2. The developer should have the flexibility to adopt the default "parent" characteristics or overwrite them as desired.
//! 3. There are common class attributes that are required in order to manage it using CRUD
//!  + `id` - The unique identifier of the object.
//!  + `created_dtm` - The unix epoch (UTC) representation of when the object was created
//!  + `modified_dtm` - The unix epoch (UTC) representation of when the object was last updated
//!  + `inactive_dtm` - The unix epoch (UTC) representation of when the object was/will be considered obsolete
//!  + `expired_dtm` - The unix epoch (UTC) representation of when the object was/will be ready for deletion
//!  + `activity` - The list of actions performed on the object
//! 4. There is common class behaviors that are required in order to manage it using CRUD
//!  + The `id` is not optional. It must be either provided or automatically generated during instantiation.
//!    This can be done by calling the `Scaffolding` trait's `id()` method
//!  + The `created_dtm` is not optional. It must be either provided or automatically generated during instantiation.
//!    This can be done by calling one of the `Scaffolding` trait's many datetime related methods, (e.g.: `now()`)  
//!  + The `modified_dtm` is not optional. It must be either provided or automatically generated during instantiation or updates to the object.
//!    This can be done by calling one of the `Scaffolding` trait's many datetime related methods, (e.g.: `now()`)
//!  + The `inactive_dtm` is not optional. It must be either provided or automatically generated during instantiation or updates to the object.
//!    This can be done by calling one of the `Scaffolding` trait's many datetime related methods, (e.g.: `add_months()` in conjuctions with `now()`)  
//!  + The `expire_dtm` is not optional. It must be either provided or automatically generated during instantiation or updates to the object.
//!    This can be done by calling one of the `Scaffolding` trait's many datetime related methods, (e.g.: `never()`)
//!  + The `activity` is required and by default is an empty list of activity
//!
//! ### Example
//! Add Scaffolding to a `struct` and `impl` `::new()` using macros and defaults
//! ```rust
//! extern crate scaffolding_core;
//!
//! use scaffolding_core::*;
//! use scaffolding_macros::*;
//! use serde_derive::{Deserialize, Serialize};
//!
//! // (1) Define the structure - Required
//! #[scaffolding_struct]
//! #[derive(Debug, Clone, Deserialize, Serialize, Scaffolding)]
//! struct MyEntity {
//!     a: bool,
//!     b: String,
//! }
//!
//! impl MyEntity {
//!     // (2) Define the constructor - Optional
//!     //     Note: Any of the Scaffodling attributes that are set here
//!     //           will not be overwritten when generated. For example
//!     //           the `id` attribute, if uncommented, would be ignored.
//!     #[scaffolding_fn]
//!     fn new(arg: bool) -> Self {
//!         let msg = format!("You said it is {}", arg);
//!         Self {
//!             // id: "my unique identitifer".to_string(),
//!             a: arg,
//!             b: msg
//!         }
//!     }
//!
//!     fn my_func(&self) -> String {
//!         "my function".to_string()
//!     }
//! }
//!
//! let mut entity = MyEntity::new(true);
//!
//! /* scaffolding attributes */
//! assert_eq!(entity.id.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
//! assert_eq!(entity.created_dtm, defaults::now());
//! assert_eq!(entity.modified_dtm, defaults::now());
//! // becomes inactive in 90 days
//! assert_eq!(entity.inactive_dtm, defaults::add_days(defaults::now(), 90));
//! // expires in 3 years
//! assert_eq!(entity.expired_dtm, defaults::add_years(defaults::now(), 3));
//!
//! /* use the activity log functionality  */
//! // (1) Log an activity
//! entity.log_activity("cancelled".to_string(), "The customer has cancelled their service".to_string());
//! // (2) Get activities
//! assert_eq!(entity.get_activity("cancelled".to_string()).len(), 1);
//!
//! // extended attributes
//! assert_eq!(entity.a, true);
//! assert_eq!(entity.b, "You said it is true");
//!
//! // extended behavior
//! assert_eq!(entity.my_func(), "my function");
//! ```
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use errors::*;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

/// Supporting Classes
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivityItem {
    // The timestamp when the action occurred
    pub created_dtm: i64,
    // The textual name of the action that occurred
    pub action: String,
    // The textual description of the action that occurred
    pub description: String,
}

impl ActivityItem {
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
    pub fn new(name: String, descr: String) -> Self {
        Self {
            created_dtm: defaults::now(),
            action: name,
            description: descr,
        }
    }

    /// This function instantiates an ActivityItem from a JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::ActivityItem;
    /// use serde_derive::Deserialize;
    ///
    /// let serialized = r#"{
    ///   "created_dtm":1711760135,
    ///   "action":"updated",
    ///   "description":"The object has been updated."
    /// }"#;
    /// let mut activity_item = ActivityItem::deserialized(&serialized.as_bytes()).unwrap();
    ///
    /// assert_eq!(activity_item.created_dtm, 1711760135);
    /// assert_eq!(activity_item.action, "updated".to_string());
    /// assert_eq!(activity_item.description, "The object has been updated.".to_string());
    /// ```
    pub fn deserialized(serialized: &[u8]) -> Result<ActivityItem, DeserializeError> {
        match serde_json::from_slice(&serialized) {
            Ok(item) => Ok(item),
            Err(err) => {
                println!("{}", err);
                Err(DeserializeError)
            }
        }
    }

    /// This function converts the ActivityItem to a serialize JSON string.
    ///
    /// #Example
    ///
    /// ```rust
    /// use scaffolding_core::ActivityItem;
    /// use serde_derive::Serialize;
    ///
    ///
    /// let mut activity_item = ActivityItem::new("updated".to_string(), "This was updated".to_string());
    /// let json = activity_item.serialize();
    ///
    /// println!("{}", json);
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    // The unique identifier of the note
    pub id: String,
    // The timestamp when the note was created
    pub created_dtm: i64,
    // The timestamp when the note was last modified
    pub modified_dtm: i64,
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
    /// use scaffolding_core::Address;
    ///
    /// fn main() {
    ///   let address = Address::new(
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
    ///   println!("{}", address.id);
    ///   println!("{}", address.created_dtm);
    ///   println!("{}", address.modified_dtm,);
    /// }
    /// ```
    pub fn new(
        category: String,
        line_1: String,
        line_2: String,
        line_3: String,
        line_4: String,
        country_code: String,
    ) -> Self {
        Self {
            id: defaults::id(),
            created_dtm: defaults::now(),
            modified_dtm: defaults::now(),
            category: category,
            line_1: line_1,
            line_2: line_2,
            line_3: line_3,
            line_4: line_4,
            country_code: country_code,
        }
    }

    /// This function instantiates an Address from a JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::Address;
    /// use serde_derive::Deserialize;
    ///
    /// let serialized = r#"{
    ///   "id":"2d624160-16b1-49ce-9b90-09a82127d6ac",
    ///   "created_dtm":1711833619,
    ///   "modified_dtm":1711833619,
    ///   "category":"shipping",
    ///   "line_1":"acmes company",
    ///   "line_2":"14 Main Street",
    ///   "line_3":"Big City, NY 038845",
    ///   "line_4":"United States",
    ///   "country_code": "USA"
    /// }"#;
    /// let mut address = Address::deserialized(&serialized.as_bytes()).unwrap();
    ///
    /// assert_eq!(address.created_dtm, 1711833619);
    /// assert_eq!(address.modified_dtm, 1711833619);
    /// assert_eq!(address.category, "shipping".to_string());
    /// ```
    pub fn deserialized(serialized: &[u8]) -> Result<Address, DeserializeError> {
        match serde_json::from_slice(&serialized) {
            Ok(item) => Ok(item),
            Err(err) => {
                println!("{}", err);
                Err(DeserializeError)
            }
        }
    }

    /// This function converts the Address to a serialize JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, Address};
    /// use serde_derive::{Serialize};
    ///
    /// let mut address = Address::new(
    ///   "shipping".to_string(),
    ///   "acmes company".to_string(),
    ///   "14 Main Street".to_string(),
    ///   "Big City, NY 038845".to_string(),
    ///   "USA".to_string(),
    ///   "USA".to_string()
    /// );
    /// println!("{}", address.serialize());
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// This function updates the Address.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, Address};
    /// use serde_derive::{Serialize};
    ///
    /// let mut address = Address::new(
    ///   "shipping".to_string(),
    ///   "acmes company".to_string(),
    ///   "14 Main Street".to_string(),
    ///   "Big City, NY 038845".to_string(),
    ///   "USA".to_string(),
    ///   "USA".to_string()
    /// );
    ///
    /// address.update(
    ///   "billing".to_string(),
    ///   "acmes company".to_string(),
    ///   "14 Main Street".to_string(),
    ///   "Big City, NY 038845".to_string(),
    ///   "USA".to_string(),
    ///   "USA".to_string());
    ///
    /// assert_eq!(address.category, "billing".to_string());
    /// ```
    pub fn update(
        &mut self,
        category: String,
        line_1: String,
        line_2: String,
        line_3: String,
        line_4: String,
        country_code: String,
    ) {
        self.category = category;
        self.line_1 = line_1;
        self.line_2 = line_2;
        self.line_3 = line_3;
        self.line_4 = line_4;
        self.country_code = country_code;
        self.modified_dtm = defaults::now();
    }
}
pub struct Countries {
    // The list of countries
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
    /// use scaffolding_core::Countries;
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
    /// use scaffolding_core::{Countries, Country};
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
    /// use scaffolding_core::{Countries, Country};
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
    /// use scaffolding_core::{Countries, Country};
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
    /// use scaffolding_core::{Countries, Country};
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
    /// use scaffolding_core::Country;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailAddress {
    // The unique identifier of the note
    pub id: String,
    // The timestamp when the note was created
    pub created_dtm: i64,
    // The timestamp when the note was last modified
    pub modified_dtm: i64,
    // The type of email address, (e.g.: Login, Personal, Work, Primary Contact, Assistant, etc.)
    pub category: String,
    // The email address
    pub address: String,
}

impl EmailAddress {
    /// This is the constructor function.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::EmailAddress;
    ///
    /// fn main() {
    ///   let email = EmailAddress::new(
    ///       "home".to_string(),
    ///       "myemail@example.com".to_string(),
    ///   );
    ///   
    ///   // scaffolding attributes
    ///   println!("{}", email.id);
    ///   println!("{}", email.created_dtm);
    ///   println!("{}", email.modified_dtm,);
    /// }
    /// ```
    pub fn new(category: String, address: String) -> Self {
        Self {
            id: defaults::id(),
            created_dtm: defaults::now(),
            modified_dtm: defaults::now(),
            category: category,
            address: address,
        }
    }

    /// This function instantiates a EmailAddress from a JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::EmailAddress;
    /// use serde_derive::Deserialize;
    ///
    /// let serialized = r#"{
    ///   "id":"2d624160-16b1-49ce-9b90-09a82127d6ac",
    ///   "created_dtm":1711833619,
    ///   "modified_dtm":1711833619,
    ///   "category":"home",
    ///   "address":"myemail@example.com"
    /// }"#;
    /// let mut email = EmailAddress::deserialized(&serialized.as_bytes()).unwrap();
    ///
    /// assert_eq!(email.created_dtm, 1711833619);
    /// assert_eq!(email.modified_dtm, 1711833619);
    /// assert_eq!(email.category, "home".to_string());
    /// ```
    pub fn deserialized(serialized: &[u8]) -> Result<EmailAddress, DeserializeError> {
        match serde_json::from_slice(&serialized) {
            Ok(item) => Ok(item),
            Err(err) => {
                println!("{}", err);
                Err(DeserializeError)
            }
        }
    }

    /// This function performs a quick check to see if the email address is properly formatted.
    /// NOTE: This is not a validation that the email address is real.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::EmailAddress;
    /// use serde_derive::Deserialize;
    ///
    /// let email = EmailAddress::new(
    ///     "home".to_string(),
    ///     "myemail@example.com".to_string(),
    /// );
    ///
    /// assert_eq!(email.is_valid(), true);
    /// ```
    pub fn is_valid(&self) -> bool {
        // use regex::Regex;
        let exp = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
        let re = Regex::new(exp).unwrap();
        re.is_match(&self.address)
    }
    /// This function converts the EmailAddress to a serialize JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, EmailAddress};
    /// use serde_derive::{Serialize};
    ///
    /// let mut email = EmailAddress::new(
    ///       "home".to_string(),
    ///       "myemail@example.com".to_string(),
    /// );
    /// println!("{}", email.serialize());
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    // The unique identifier of the note
    pub id: String,
    // The timestamp when the note was created
    pub created_dtm: i64,
    // The timestamp when the note was last modified
    pub modified_dtm: i64,
    // The identifier of the author of the note
    pub author: String,
    // The identifier of access rule of the note, (e.g.: public, internal, confidential)
    pub access: String,
    // The comment of the note
    pub content: Vec<u8>,
}

impl Note {
    /// This is the constructor function.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, Note};
    ///
    /// let note = Note::new("fsmith".to_string(), "This was updated".as_bytes().to_vec(), None);
    /// ```
    pub fn new(auth: String, cont: Vec<u8>, acc: Option<String>) -> Self {
        Self {
            id: defaults::id(),
            created_dtm: defaults::now(),
            modified_dtm: defaults::now(),
            author: auth,
            access: match acc {
                Some(a) => a,
                None => defaults::access(),
            },
            content: cont,
        }
    }

    /// This function returns the content of the note as a string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, Note};
    ///
    /// let note = Note::new("fsmith".to_string(), "This was updated".as_bytes().to_vec(), None);
    /// assert_eq!(note.content_as_string().unwrap(), "This was updated".to_string());
    /// ```
    pub fn content_as_string(&self) -> Result<String, String> {
        String::from_utf8(self.content.clone())
            .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
    }

    /// This function instantiates an ActivityItem from a JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::Note;
    /// use serde_derive::Deserialize;
    ///
    /// let serialized = r#"{
    ///   "id":"2d624160-16b1-49ce-9b90-09a82127d6ac",
    ///   "created_dtm":1711833619,
    ///   "modified_dtm":1711833619,
    ///   "author":"fsmith",
    ///   "access":"public",
    ///   "content":[84,104,105,115,32,119,97,115,32,117,112,100,97,116,101,100]
    /// }"#;
    /// let mut note = Note::deserialized(&serialized.as_bytes()).unwrap();
    ///
    /// assert_eq!(note.created_dtm, 1711833619);
    /// assert_eq!(note.modified_dtm, 1711833619);
    /// assert_eq!(note.author, "fsmith".to_string());
    /// assert_eq!(note.access, "public".to_string());
    /// assert_eq!(note.content, "This was updated".as_bytes().to_vec());
    /// ```
    pub fn deserialized(serialized: &[u8]) -> Result<Note, DeserializeError> {
        match serde_json::from_slice(&serialized) {
            Ok(item) => Ok(item),
            Err(err) => {
                println!("{}", err);
                Err(DeserializeError)
            }
        }
    }

    /// This function converts the ActivityItem to a serialize JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, Note};
    /// use serde_derive::{Serialize};
    ///
    /// let mut note = Note::new("fsmith".to_string(), "This was updated".as_bytes().to_vec(), None);
    /// println!("{}", note.serialize());
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// This function updates the note and sets the modified_dtm.
    /// The modified_dtm will not be changed if the attributes are written to directly.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, Note};
    /// use serde_derive::Deserialize;
    ///
    /// let serialized = r#"{
    ///   "id":"2d624160-16b1-49ce-9b90-09a82127d6ac",
    ///   "created_dtm":1711833619,
    ///   "modified_dtm":1711833619,
    ///   "author":"fsmith",
    ///   "access":"public",
    ///   "content":[84,104,105,115,32,119,97,115,32,117,112,100,97,116,101,100]
    /// }"#;
    /// let mut note = Note::deserialized(&serialized.as_bytes()).unwrap();
    /// let first_modified = note.modified_dtm.clone();
    ///
    /// note.update("fsmith".to_string(), "This was updated again".as_bytes().to_vec(), Some("private".to_string()));
    ///
    /// assert_eq!(note.author, "fsmith".to_string());
    /// assert_eq!(note.access, "private".to_string());
    /// assert_eq!(note.content, "This was updated again".as_bytes().to_vec());
    /// assert!(note.modified_dtm > first_modified);
    /// ```
    pub fn update(&mut self, auth: String, cont: Vec<u8>, acc: Option<String>) {
        self.author = auth;
        self.content = cont;
        self.access = match acc {
            Some(a) => a,
            None => self.access.clone(),
        };
        self.modified_dtm = defaults::now();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhoneNumber {
    // The unique identifier of the note
    pub id: String,
    // The timestamp when the note was created
    pub created_dtm: i64,
    // The timestamp when the note was last modified
    pub modified_dtm: i64,
    // The type of address, (e.g.: Login, Personal, Work, Primary Contact, Assistant, etc.)
    pub category: String,
    // The phone number
    pub number: String,
    // The country code of the phone number (Use Alpha 3 codes)
    pub country_code: String,
}

impl PhoneNumber {
    /// This is the constructor function.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///
    /// use scaffolding_core::PhoneNumber;
    ///
    /// fn main() {
    ///   let phone = PhoneNumber::new(
    ///       "home".to_string(),
    ///       "8482493561".to_string(),
    ///       "USA".to_string(),
    ///   );
    ///   
    ///   // scaffolding attributes
    ///   println!("{}", phone.id);
    ///   println!("{}", phone.created_dtm);
    ///   println!("{}", phone.modified_dtm,);
    /// }
    /// ```
    pub fn new(category: String, number: String, country_code: String) -> Self {
        Self {
            id: defaults::id(),
            created_dtm: defaults::now(),
            modified_dtm: defaults::now(),
            category: category,
            number: number,
            country_code: country_code,
        }
    }

    /// This function instantiates a PhoneNumber from a JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::PhoneNumber;
    /// use serde_derive::Deserialize;
    ///
    /// let serialized = r#"{
    ///   "id":"2d624160-16b1-49ce-9b90-09a82127d6ac",
    ///   "created_dtm":1711833619,
    ///   "modified_dtm":1711833619,
    ///   "category":"home",
    ///   "number":"8482493561",
    ///   "country_code": "USA"
    /// }"#;
    /// let mut phone = PhoneNumber::deserialized(&serialized.as_bytes()).unwrap();
    ///
    /// assert_eq!(phone.created_dtm, 1711833619);
    /// assert_eq!(phone.modified_dtm, 1711833619);
    /// assert_eq!(phone.category, "home".to_string());
    /// ```
    pub fn deserialized(serialized: &[u8]) -> Result<PhoneNumber, DeserializeError> {
        match serde_json::from_slice(&serialized) {
            Ok(item) => Ok(item),
            Err(err) => {
                println!("{}", err);
                Err(DeserializeError)
            }
        }
    }

    /// This function converts the PhoneNumber to a serialize JSON string.
    ///
    /// #Example
    ///
    /// ```rust     
    /// use scaffolding_core::{defaults, PhoneNumber};
    /// use serde_derive::{Serialize};
    ///
    /// let mut phone = PhoneNumber::new(
    ///       "home".to_string(),
    ///       "8482493561".to_string(),
    ///       "USA".to_string(),
    /// );
    /// println!("{}", phone.serialize());
    /// ```
    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// The core behavior of a Scaffolding object
pub trait Scaffolding {
    type Item;
    /// This function adds a ActivityItem to the activity log
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    /// use scaffolding_macros::*;
    ///
    /// #[scaffolding_struct]
    /// #[derive(Clone, Debug, Scaffolding)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    ///
    /// entity.log_activity("cancelled".to_string(), "The customer has cancelled their service".to_string());
    /// assert_eq!(entity.activity.len(), 1);
    /// ```
    fn log_activity(&mut self, name: String, descr: String);

    /// This function retrieves all the ActivityItems that have the specified action (name)
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    /// use scaffolding_macros::*;
    ///
    /// #[scaffolding_struct]
    /// #[derive(Clone, Debug, Scaffolding)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    ///
    /// entity.log_activity("ordered".to_string(), "The customer has place the order".to_string());
    /// entity.log_activity("cancelled".to_string(), "The customer has cancelled their service".to_string());
    /// assert_eq!(entity.get_activity("cancelled".to_string()).len(), 1);
    /// ```
    fn get_activity(&self, name: String) -> Vec<ActivityItem>;

    /// This function instantiates an entity from a JSON string.
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    /// use scaffolding_macros::*;
    /// use serde_derive::Deserialize;
    ///
    /// #[scaffolding_struct]
    /// #[derive(Clone, Debug, Deserialize, Scaffolding)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let json = r#"{
    ///     "id":"b4d6c6db-7468-400a-8536-a5e83b1f2bdc",
    ///     "created_dtm":1711802687,
    ///     "modified_dtm":1711802687,
    ///     "inactive_dtm":1719578687,
    ///     "expired_dtm":1806410687,
    ///     "activity":[
    ///         {
    ///             "created_dtm":1711802687,
    ///             "action":"updated",
    ///             "description":"The object has been updated"
    ///         },
    ///         {
    ///             "created_dtm":1711802687,
    ///             "action":"updated",
    ///             "description":"The object has been updated"
    ///         },
    ///         {
    ///             "created_dtm":1711802687,
    ///             "action":"cancelled",
    ///             "description":"The object has been cancelled"
    ///         }
    ///         ]
    ///     }"#;
    /// let deserialized = MyEntity::deserialized::<MyEntity>(json.as_bytes()).unwrap();
    ///
    /// assert_eq!(deserialized.id, "b4d6c6db-7468-400a-8536-a5e83b1f2bdc");
    /// assert_eq!(deserialized.activity.len(), 3);  
    ///
    /// ```
    fn deserialized<Item: DeserializeOwned>(serialized: &[u8]) -> Result<Item, DeserializeError> {
        match serde_json::from_slice::<Item>(&serialized) {
            Ok(item) => Ok(item),
            Err(err) => {
                println!("{}", err);
                Err(DeserializeError)
            }
        }
    }

    /// This function converts the entity to a serialize JSON string.
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    /// use scaffolding_macros::*;
    /// use serde_derive::Serialize;
    ///
    /// #[scaffolding_struct]
    /// #[derive(Clone, Debug, Serialize, Scaffolding)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let json_string = entity.serialize();
    ///
    /// println!("{}", json_string);
    /// ```
    fn serialize(&mut self) -> String
    where
        Self: Serialize,
    {
        serde_json::to_string(&self).unwrap()
    }
}

/// The addresses behavior of a Scaffolding object
pub trait ScaffoldingAddresses {
    /// Retrieves a related Address to the Entity based on the specified id.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_address(
    ///     "shipping".to_string(),
    ///     "acmes company".to_string(),
    ///     "14 Main Street".to_string(),
    ///     "Big City, NY 038845".to_string(),
    ///     "USA".to_string(),
    ///     "USA".to_string(),
    /// );
    ///
    /// assert_eq!(entity.get_address(id).unwrap().category, "shipping".to_string());
    /// ```    
    fn get_address(&self, id: String) -> Option<&Address>;

    /// Insert or updates a related Address to the Entity and returns the id of the Address.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let _ = entity.insert_address(
    ///     "shipping".to_string(),
    ///     "acmes company".to_string(),
    ///     "14 Main Street".to_string(),
    ///     "Big City, NY 038845".to_string(),
    ///     "USA".to_string(),
    ///     "USA".to_string(),
    /// );
    ///
    /// assert_eq!(entity.addresses.len(), 1);
    /// ```
    fn insert_address(
        &mut self,
        category: String,
        line_1: String,
        line_2: String,
        line_3: String,
        line_4: String,
        country_code: String,
    ) -> String;

    /// Insert or updates a related Address to the Entity and returns the id of the Address for reference.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_address(
    ///     "shipping".to_string(),
    ///     "acmes company".to_string(),
    ///     "14 Main Street".to_string(),
    ///     "Big City, NY 038845".to_string(),
    ///     "USA".to_string(),
    ///     "USA".to_string(),
    /// );
    ///
    /// entity.modify_address(
    ///     id.clone(),
    ///     "billing".to_string(),
    ///     "acmes company".to_string(),
    ///     "14 Main Street".to_string(),
    ///     "Big City, NY 038845".to_string(),
    ///     "USA".to_string(),
    ///     "USA".to_string(),);
    ///
    /// assert_eq!(entity.get_address(id).unwrap().category, "billing".to_string());
    /// ```
    fn modify_address(
        &mut self,
        id: String,
        category: String,
        line_1: String,
        line_2: String,
        line_3: String,
        line_4: String,
        country_code: String,
    );

    /// Retrieves all the Addresses with the specified category.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let address = entity.insert_address(
    ///     "shipping".to_string(),
    ///     "acmes company".to_string(),
    ///     "14 Main Street".to_string(),
    ///     "Big City, NY 038845".to_string(),
    ///     "USA".to_string(),
    ///     "USA".to_string(),
    /// );
    ///
    /// assert_eq!(entity.search_addresses_by_category("shipping".to_string()).len(), 1);
    /// ```
    fn search_addresses_by_category(&self, category: String) -> Vec<Address>;

    /// Removes a related Address to the Entity.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_address(
    ///     "shipping".to_string(),
    ///     "acmes company".to_string(),
    ///     "14 Main Street".to_string(),
    ///     "Big City, NY 038845".to_string(),
    ///     "USA".to_string(),
    ///     "USA".to_string(),
    /// );
    /// assert_eq!(entity.addresses.len(), 1);
    ///
    /// entity.remove_address(id);
    /// assert_eq!(entity.addresses.len(), 0);
    /// ```
    fn remove_address(&mut self, id: String);
}

/// The email address behavior of a Scaffolding object
pub trait ScaffoldingEmailAddresses {
    /// Retrieves a related EmailAddress based on the specific id.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("email_addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingEmailAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("email_addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_email_address(
    ///     "home".to_string(),
    ///     "myemail@example.com".to_string(),
    /// );
    ///
    /// assert_eq!(entity.get_email_address(id).unwrap().address, "myemail@example.com".to_string());
    /// ```
    fn get_email_address(&self, id: String) -> Option<&EmailAddress>;

    /// Adds a related PhoneNumber to the Entity and returns the id for reference.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("email_addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingEmailAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("email_addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let _ = entity.insert_email_address(
    ///     "home".to_string(),
    ///     "myemail@example.com".to_string(),
    /// );
    ///
    /// assert_eq!(entity.email_addresses.len(), 1);
    /// ```
    fn insert_email_address(&mut self, category: String, address: String) -> String;

    /// Retrieves all the EmailAddress with the specified category.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("email_addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingEmailAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("email_addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let _ = entity.insert_email_address(
    ///     "home".to_string(),
    ///     "myemail@example.com".to_string(),
    /// );
    ///
    /// assert_eq!(entity.search_email_addresses_by_category("home".to_string()).len(), 1);
    /// ```
    fn search_email_addresses_by_category(&self, category: String) -> Vec<EmailAddress>;

    /// Removes a related EmailAddress to the Entity.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("email_addresses")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingEmailAddresses)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("email_addresses")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_email_address(
    ///     "home".to_string(),
    ///     "myemail@example.com".to_string(),
    /// );
    /// assert_eq!(entity.email_addresses.len(), 1);
    ///
    /// entity.remove_email_address(id);
    /// assert_eq!(entity.email_addresses.len(), 0);
    /// ```
    fn remove_email_address(&mut self, id: String);
}

/// The notes behavior of a Scaffolding object
pub trait ScaffoldingNotes {
    /// Retrieves a related Note based on the specific id.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("notes")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingNotes)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("notes")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "This was updated".as_bytes().to_vec(),
    ///     None,
    /// );
    ///
    /// assert_eq!(entity.get_note(id).unwrap().content_as_string().unwrap(), "This was updated".to_string());
    /// ```
    fn get_note(&self, id: String) -> Option<&Note>;

    /// Inserts a related Note.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("notes")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingNotes)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("notes")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "This was updated".as_bytes().to_vec(),
    ///     None,
    /// );
    ///
    /// assert_eq!(entity.notes.len(), 1);
    /// ```
    fn insert_note(&mut self, auth: String, cont: Vec<u8>, acc: Option<String>) -> String;

    /// Updates a related Note based on the specified id.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("notes")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingNotes)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("notes")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "This was updated".as_bytes().to_vec(),
    ///     None,
    /// );
    ///
    /// entity.modify_note(
    ///     id.clone(),
    ///     "fsmith".to_string(),
    ///     "This was updated again".as_bytes().to_vec(),
    ///     Some("private".to_string()),
    /// );
    /// ```
    fn modify_note(&mut self, id: String, auth: String, cont: Vec<u8>, acc: Option<String>);

    /// Searches the notes for specific string and returns all the notes that were found.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("notes")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingNotes)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("notes")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    ///
    /// let _ = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "This was updated".as_bytes().to_vec(),
    ///     None,
    /// );
    /// let _ = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "Something to find here".as_bytes().to_vec(),
    ///     None,
    /// );
    /// let _ = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "Nonething to find here".as_bytes().to_vec(),
    ///     Some("private".to_string()),
    /// );
    ///  
    /// let search_results = entity.search_notes("thing".to_string());
    ///
    /// assert_eq!(search_results.len(), 2);
    /// ```
    fn search_notes(&mut self, search: String) -> Vec<Note>;

    /// Removes a note for specific id.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("notes")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingNotes)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("notes")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    ///
    /// let _ = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "This was updated".as_bytes().to_vec(),
    ///     None,
    /// );
    /// let id = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "Something to find here".as_bytes().to_vec(),
    ///     None,
    /// );
    /// let _ = entity.insert_note(
    ///     "fsmith".to_string(),
    ///     "Nonething to find here".as_bytes().to_vec(),
    ///     Some("private".to_string()),
    /// );
    ///  
    /// entity.remove_note(id);
    ///
    /// assert_eq!(entity.notes.len(), 2);
    /// ```
    fn remove_note(&mut self, id: String);
}

/// The phone number behavior of a Scaffolding object
pub trait ScaffoldingPhoneNumbers {
    /// Retrieves a related PhoneNumber based on the specific id.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("phone_numbers")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingPhoneNumbers)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("phone_numbers")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_phone_number(
    ///     "home".to_string(),
    ///     "8482493561".to_string(),
    ///     "USA".to_string(),
    /// );
    ///
    /// assert_eq!(entity.get_phone_number(id).unwrap().number, "8482493561".to_string());
    /// ```
    fn get_phone_number(&self, id: String) -> Option<&PhoneNumber>;

    /// Adds a related PhoneNumber to the Entity and returns the id for reference.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("phone_numbers")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingPhoneNumbers)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("phone_numbers")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let _ = entity.insert_phone_number(
    ///     "home".to_string(),
    ///     "8482493561".to_string(),
    ///     "USA".to_string(),
    /// );
    ///
    /// assert_eq!(entity.phone_numbers.len(), 1);
    /// ```
    fn insert_phone_number(
        &mut self,
        category: String,
        number: String,
        country_code: String,
    ) -> String;

    /// Retrieves all the PhoneNumber with the specified category.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("phone_numbers")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingPhoneNumbers)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("phone_numbers")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let _ = entity.insert_phone_number(
    ///     "home".to_string(),
    ///     "8482493561".to_string(),
    ///     "USA".to_string(),
    /// );
    ///
    /// assert_eq!(entity.search_phone_numbers_by_category("home".to_string()).len(), 1);
    /// ```
    fn search_phone_numbers_by_category(&self, category: String) -> Vec<PhoneNumber>;

    /// Removes a related PhoneNumber to the Entity.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate scaffolding_core;
    ///     
    /// use scaffolding_core::*;
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    /// use std::collections::BTreeMap;
    ///
    /// #[scaffolding_struct("phone_numbers")]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding, ScaffoldingPhoneNumbers)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("phone_numbers")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    /// let id = entity.insert_phone_number(
    ///     "home".to_string(),
    ///     "8482493561".to_string(),
    ///     "USA".to_string(),
    /// );
    /// assert_eq!(entity.phone_numbers.len(), 1);
    ///
    /// entity.remove_phone_number(id);
    /// assert_eq!(entity.phone_numbers.len(), 0);
    /// ```
    fn remove_phone_number(&mut self, id: String);
}

/// The tagging behavior of a Scaffolding object
pub trait ScaffoldingTags {
    /// This function adds a tag to the object
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding, ScaffoldingTags};
    /// use scaffolding_macros::*;
    ///
    /// #[scaffolding_struct("tags")]
    /// #[derive(Clone, Debug, Scaffolding, ScaffoldingTags)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("tags")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    ///
    /// entity.add_tag("tag_1".to_string());
    /// // ignore any duplicates
    /// entity.add_tag("tag_1".to_string());
    /// entity.add_tag("tag_2".to_string());
    /// entity.add_tag("tag_3".to_string());
    ///
    /// assert_eq!(entity.tags.len(), 3);
    /// ```
    fn add_tag(&mut self, tag: String);

    /// This function determines if the object has a specific tag
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding, ScaffoldingTags};
    /// use scaffolding_macros::*;
    ///
    /// #[scaffolding_struct("tags")]
    /// #[derive(Clone, Debug, Scaffolding, ScaffoldingTags)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("tags")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    ///
    /// entity.add_tag("tag_1".to_string());
    ///
    /// assert!(entity.has_tag("tag_1".to_string()));
    /// ```
    fn has_tag(&self, tag: String) -> bool;

    /// This function removes a specific tag from the object
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding, ScaffoldingTags};
    /// use scaffolding_macros::*;
    ///
    /// #[scaffolding_struct("tags")]
    /// #[derive(Clone, Debug, Scaffolding, ScaffoldingTags)]
    /// struct MyEntity {}
    ///
    /// impl MyEntity {
    ///     #[scaffolding_fn("tags")]
    ///     fn new() -> Self {
    ///         Self {}
    ///     }
    /// }
    ///
    /// let mut entity = MyEntity::new();
    ///
    /// entity.add_tag("tag_1".to_string());
    /// assert_eq!(entity.tags.len(), 1);
    /// entity.remove_tag("tag_1".to_string());
    /// assert_eq!(entity.tags.len(), 0);
    /// ```
    fn remove_tag(&mut self, tag: String);
}

// modules
pub mod defaults;
pub mod errors;

#[cfg(test)]
mod tests {
    use crate::{defaults, ActivityItem};

    fn get_actionitem() -> ActivityItem {
        ActivityItem::new(
            "updated".to_string(),
            "The object has been updated.".to_string(),
        )
    }
    #[test]
    fn test_activityitem_new() {
        let ai = get_actionitem();

        assert_eq!(ai.created_dtm, defaults::now());
        assert_eq!(ai.action, "updated".to_string());
        assert_eq!(ai.description, "The object has been updated.".to_string());
    }

    #[test]
    fn test_activityitem_serialization() {
        let serialized = r#"{"created_dtm":1711760135,"action":"updated","description":"The object has been updated."}"#;
        let mut ai = ActivityItem::deserialized(&serialized.as_bytes()).unwrap();

        assert_eq!(ai.created_dtm, 1711760135);
        assert_eq!(ai.action, "updated".to_string());
        assert_eq!(ai.description, "The object has been updated.".to_string());
        assert_eq!(ai.serialize(), serialized);
    }
}
