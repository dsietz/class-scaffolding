[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Docs.rs](https://docs.rs/scaffolding-core/badge.svg)](https://docs.rs/scaffolding-core)
![Build/Test](https://github.com/dsietz/scaffolding-core/actions/workflows/master.yaml/badge.svg)
[![Discussions](https://img.shields.io/github/discussions/dsietz/scaffolding-core)](https://github.com/dsietz/scaffolding-core/discussions)

# Scaffolding Core

For software development teams who appreciate a kick-start to their object oriented development, this scaffolding library is a light-weight module that provides the basic structures and behavior that is the building block of all class intantiated objects. Unlike the practice of writing classes with the various approaches for building common functionality, this open source library helps you inherit these cross-class commonalities so you can focus on the differentiator that define your class.   

---

### Table of Contents
- [Scaffolding Core](#scaffolding-core)
    - [Table of Contents](#table-of-contents)
  - [What's New](#whats-new)
  - [Examples](#examples)
  - [Usage](#usage)
  - [How to Contribute](#how-to-contribute)
  - [License](#license)

---

## What's New
| :warning: Please Note!                                                                  |
| ----------------------------------------------------------------------------- |
| This crate is in an `beta` release phase and is only intended as experimental.|

**0.6.0**
+ [Provide the ability to manage phone numbers](https://github.com/dsietz/scaffolding-core/issues/36)
+ [Provide the ability to manage email addresses](https://github.com/dsietz/scaffolding-core/issues/37)

## Examples
```rust
cargo run --example person
```

## Usage
Add Scaffolding to a `struct` and `impl` `::new()` using macros and defaults

```rust
extern crate scaffolding_core;

use scaffolding_core::*;
use scaffolding_macros::*;
use serde_derive::{Deserialize, Serialize};
// Required for scaffolding metadata functionality
use std::collections::BTreeMap;

// (1) Define the structure - Required
#[scaffolding_struct("addresses","metadata","notes","tags")]
#[derive(Debug, Clone, Deserialize, Serialize, Scaffolding, ScaffoldingAddresses, ScaffoldingNotes, ScaffoldingTags)]
struct MyEntity {
    a: bool,
    b: String,
}

impl MyEntity {
    // (2) Define the constructor - Optional
    //     Note: Any of the Scaffodling attributes that are set here 
    //           will not be overwritten when generated. For example
    //           the `id` attribute, if uncommented, would be ignored.
    #[scaffolding_fn("addresses","metadata","notes","tags")]
    fn new(arg: bool) -> Self {
        let msg = format!("You said it is {}", arg);
        Self {
            // id: "my unique identitifer".to_string(),
            a: arg,
            b: msg
        }
    }

    fn my_func(&self) -> String {
        "my function".to_string()
    }
}

let mut entity = MyEntity::new(true);

/* scaffolding attributes */
assert_eq!(entity.id.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
assert_eq!(entity.created_dtm, defaults::now());
assert_eq!(entity.modified_dtm, defaults::now());
// becomes inactive in 90 days
assert_eq!(entity.inactive_dtm, defaults::add_days(defaults::now(), 90));
// expires in 3 years
assert_eq!(entity.expired_dtm, defaults::add_years(defaults::now(), 3));

/* use the activity log functionality  */
// (1) Log an activity
entity.log_activity("cancelled".to_string(), "The customer has cancelled their service".to_string());
// (2) Get activities
assert_eq!(entity.get_activity("cancelled".to_string()).len(), 1);

/* use the addresses functionality */
// (1) Add an address
let addrShipping = entity.add_address(
    "shipping".to_string(),
    "acmes company".to_string(),
    "14 Main Street".to_string(),
    "Big City, NY 038845".to_string(),
    "USA".to_string(),
    "USA".to_string(),
);
let addrBilling = entity.add_address(
    "billing".to_string(),
    "acmes company".to_string(),
    "14 Main Street".to_string(),
    "Big City, NY 038845".to_string(),
    "USA".to_string(),
    "USA".to_string(),

let addrHome = entity.add_address(
    "home".to_string(),
    "Peter Petty".to_string(),
    "23 Corner Lane".to_string(),
    "Tiny Town, VT 044567".to_string(),
    "USA".to_string(),
    "USA".to_string(),
);
// (2) Find addresses based on the category
let shipping_addresses = entity.addresses_by_category("shipping".to_string());
// (3) Remove an address
entity.remove_address(addrBilling.id);

/* use the phone number functionality */
// (1) Add a phone number
let phoneHome = entity.add_phone_number(
    "home".to_string(),
    "8482493561".to_string(),
    "USA".to_string(),
);
let phoneWork = entity.add_phone_number(
    "work".to_string(),
    "2223330000".to_string(),
    "USA".to_string(),
);
// (2) Find phone number based on the category
let home_phone = entity.phone_numbers_by_category("home".to_string());
// (3) Remove an address
entity.remove_phone_number(phoneWork.id);

/* use the notes functionality */
// (1) Insert a note
let note_id = entity.insert_note(
  "fsmith".to_string(),
  "This was updated".as_bytes().to_vec(),
  None,
);
// (2) Modify the note
entity.modify_note(
  note_id.clone(),
  "fsmith".to_string(),
  "This was updated again".as_bytes().to_vec(),
  Some("private".to_string()),
);
// (3) Read the note's content
let read_note = entity.get_note(note_id.clone()).unwrap().content_as_string().unwrap();
println!("{}", read_note);
// (4) Search for notes that contain the word `updated`
let search_results = entity.search_notes("updated".to_string());
assert_eq!(search_results.len(), 1);
// (5) Delete the note
entity.remove_note(note_id);

/* use the metadata functionality
   Note: `memtadata` is a BTreeMap<String, String>
          https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
*/
entity.metadata.insert("field_1".to_string(), "myvalue".to_string());
assert_eq!(entity.metadata.len(), 1);

// manage tags
entity.add_tag("tag_1".to_string());
entity.add_tag("tag_2".to_string());
entity.add_tag("tag_3".to_string());
assert!(entity.has_tag("tag_1".to_string()));
entity.remove_tag("tag_2".to_string());
assert_eq!(entity.tags.len(), 2);

/* extended attributes */
assert_eq!(entity.a, true);
assert_eq!(entity.b, "You said it is true");

/* extended behavior */
assert_eq!(entity.my_func(), "my function");
```

## How to Contribute

Details on how to contribute can be found in the [CONTRIBUTING](./CONTRIBUTING.md) file.

## License

The `scaffolding-core` project is primarily distributed under the terms of the Apache License (Version 2.0).

See [LICENSE-APACHE "Apache License](./LICENSE-APACHE) for details.