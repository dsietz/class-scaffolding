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
//!    as well as append the "child" specific data structure and behavior
//! 2. The developer should have the flexibility to adopt the default "parent" characteristics or overwrite them as desired.
//! 3. There are common class attributes that are required in order to manage it using CRUD
//!  + `id` - The unique identifier of the object.
//!  + `created_dtm` - The unix epoch (UTC) representation of when the object was created
//!  + `modified_dtm` - The unix epoch (UTC) representation of when the object was last updated
//!  + `inactive_dtm` - The unix epoch (UTC) representation of when the object was/will be considered obsolete
//!  + `expired_dtm` - The unix epoch (UTC) representation of when the object was/will be ready for deletion
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
//!
//! ### Example
//! Add Scaffolding to a `struct` and `impl` using macros and defaults
//! ```rust
//! extern crate scaffolding_core;
//!
//! use scaffolding_core::{defaults};
//! use scaffolding_macros::*;
//! // Required for scaffolding metadata functionality
//! use std::collections::BTreeMap;
//!
//! #[scaffolding_struct("metadata")]
//! #[derive(Debug, Clone)]
//! struct MyEntity {
//!     a: bool,
//!     b: String,
//! }
//!
//! impl MyEntity {
//!     #[scaffolding_fn("metadata")]
//!     fn new(arg: bool) -> Self {
//!         let msg = format!("You said it is {}", arg);
//!         Self {
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
//! println!("{:?}", entity);
//!
//! // scaffolding attributes
//! assert_eq!(entity.id.len(), 36);
//! assert_eq!(entity.created_dtm, defaults::now());
//! assert_eq!(entity.modified_dtm, defaults::now());
//! // becomes inactive in 90 days
//! assert_eq!(entity.inactive_dtm, defaults::add_days(defaults::now(), 90));
//! // expires in 3 years
//! assert_eq!(entity.expired_dtm, defaults::add_years(defaults::now(), 3));
//! // use the metadata functionality
//! entity.metadata.insert("field_1".to_string(), "myvalue".to_string());
//! assert_eq!(entity.metadata.len(), 1);
//!
//! // extended attributes
//! assert_eq!(entity.a, true);
//! assert_eq!(entity.b, "You said it is true");
//!
//! // extended behavior
//! assert_eq!(entity.my_func(), "my function");
//! ```

/// The core behavior of an Scaffolding object
pub trait Scaffolding {}

pub mod defaults;
