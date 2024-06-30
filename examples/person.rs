extern crate scaffolding_core;

use scaffolding_core::*;

#[scaffolding_struct(
    "addresses",
    "email_addresses",
    "metadata",
    "notes",
    "phone_numbers",
    "tags"
)]
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
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    #[scaffolding_fn(
        "addresses",
        "email_addresses",
        "metadata",
        "notes",
        "phone_numbers",
        "tags"
    )]
    fn new(first: String, last: String) -> Self {
        Self {
            first_name: first,
            last_name: last,
        }
    }
    fn full_name(&self) -> String {
        format!("{}, {}", self.last_name, self.first_name)
    }
}

fn main() {
    let mut person = Person::new("John".to_string(), "Smith".to_string());

    println!("My name is {}. My id is {}", person.full_name(), person.id);
    println!("Serialized json ...");
    println!("{}", person.serialize());

    let doppelganger = Person::deserialized(person.serialize().as_bytes()).unwrap();
    println!(
        "I'm also named {} and my id is the same: {}",
        doppelganger.full_name(),
        doppelganger.id
    );
}
