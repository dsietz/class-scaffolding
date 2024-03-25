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
//!
//! ```rust
//! extern crate scaffolding_core;
//!
//! use scaffolding_core::{defaults, Scaffolding};
//! use scaffolding_macros::*;
//!
//! #[scaffolding_entity]
//! #[derive(Debug, Clone, Scaffolding)]
//! struct MyEntity {
//!     b: bool,
//! }
//!
//! impl MyEntity {
//!     fn new(arg: bool) -> Self {
//!         Self {
//!             id: defaults::id(),
//!             created_dtm: defaults::now(),
//!             modified_dtm: defaults::now(),
//!             inactive_dtm: defaults::add_months(defaults::now(), 12),
//!             expired_dtm: defaults::add_years(defaults::now(), 3),
//!             b: arg,
//!         }
//!     }
//!
//!     fn my_func(&self) -> String {
//!         "my function".to_string()
//!     }
//! }
//!
//! let entity = MyEntity::new(true);
//! println!("{:?}", entity);
//!
//! // extended attributes
//! assert_eq!(entity.b, true);
//!
//! // extended behavior
//! assert_eq!(entity.my_func(), "my function");
//! ```

/// The core behavior of an Scaffolding object
pub trait Scaffolding {}

pub mod defaults;
