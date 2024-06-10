extern crate scaffolding_core;

use scaffolding_core::*;
use scaffolding_macros::*;
use serde_derive::{Deserialize, Serialize};
// Required for scaffolding metadata functionality
use std::collections::BTreeMap;

// (1) Define the structure - Required
#[scaffolding_struct("addresses", "metadata", "notes", "phone_numbers", "tags")]
#[derive(
    Debug,
    Clone,
    Deserialize,
    Serialize,
    Scaffolding,
    ScaffoldingAddresses,
    ScaffoldingNotes,
    ScaffoldingPhoneNumbers,
    ScaffoldingTags,
)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // (2) Define the constructor
    //     Note: Any of the Scaffodling attributes that are set here
    //           will not be overwritten when generated.
    #[scaffolding_fn("addresses", "metadata", "notes", "phone_numbers", "tags")]
    fn new(first: String, last: String) -> Self {
        Self {
            // id: "my unique identitifer".to_string(),
            first_name: first,
            last_name: last,
        }
    }

    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn shipping_label(&self) -> String {
        let shipping = &self.search_addresses_by_category("shipping".to_string())[0];
        format!(
            "{}\n{}\n{}\n{}\n",
            shipping.line_1, shipping.line_2, shipping.line_3, shipping.line_4
        )
    }

    fn shipping_method(&self) -> String {
        let meta = match self.metadata.get("priority mail") {
            Some(sm) => match sm.as_str() {
                "true" => true,
                _ => false,
            },
            None => false,
        };
        match meta {
            true => "Overnight".to_string(),
            false => "First Class".to_string(),
        }
    }

    fn vip_customer(&mut self, vip: bool) {
        match vip {
            true => self.add_tag("VIP".to_string()),
            false => {
                // do nothing
            }
        }
        self.metadata.insert(
            "priority mail".to_string(),
            match self.has_tag("VIP".to_string()) {
                true => "true".to_string(),
                false => "false".to_string(),
            },
        );
    }
}

fn main() {
    let mut person = Person::new("Polly".to_string(), "Pocket".to_string());

    // addresses functionality
    let countries = Countries::new();
    let country = countries
        .get_country_by_iso_3_code("USA".to_string())
        .unwrap();
    let _ = person.insert_address(
        "billing".to_string(),
        "acmes company".to_string(),
        "14 Main Street".to_string(),
        "Big City, NY 038845".to_string(),
        country.name.clone(),
        country.iso_3_code.clone(),
    );
    let _ = person.insert_address(
        "shipping".to_string(),
        person.name(),
        "23 Corner Lane".to_string(),
        "Tiny Town, VT 044567".to_string(),
        country.name.clone(),
        country.iso_3_code.clone(),
    );

    // phone number functionality
    let _ = person.insert_phone_number(
        "home".to_string(),
        "8482493561".to_string(),
        "USA".to_string(),
    );
    let _ = person.insert_phone_number(
        "work".to_string(),
        "2223330000".to_string(),
        "USA".to_string(),
    );

    // using tagging and metadata
    person.vip_customer(true);

    // use notes
    let note_id = person.insert_note(
        "shipping".to_string(),
        format!(
            "The package has been shipping to\n\n{}\nshipping method: {}\ncontact information\n phone: +{} {}\n",
            person.shipping_label(),
            person.shipping_method(),
            countries
                .get_country_by_iso_3_code(
                    person.search_phone_numbers_by_category("home".to_string())[0].country_code.clone()
                )
                .unwrap()
                .phone_code,
            person.search_phone_numbers_by_category("home".to_string())[0].number,
        )
        .as_bytes()
        .to_vec(),
        None,
    );

    println!(
        "{}",
        person
            .get_note(note_id)
            .unwrap()
            .content_as_string()
            .unwrap()
    );
}
