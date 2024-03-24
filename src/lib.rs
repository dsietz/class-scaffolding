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
//! I. A class that `extends` the "Scaffolding class" should inherate all the "parent" data structure and behavior,
//!    as well as append the "child" specific data structure and behavior
//! II. The developer should have the flexibility to adopt the default "parent" characteristics or overwrite them as desired.
//! III. There are common class attributes that are required in order to manage it using CRUD
//!  + `id` - The unique identifier of the object.
//!  + `created_dtm` - The unix epoch (UTC) representation of when the object was created
//!  + `modified_dtm` - The unix epoch (UTC) representation of when the object was last updated
//!  + `inactive_dtm` - The unix epoch (UTC) representation of when the object was/will be considered obsolete
//!  + `expired_dtm` - The unix epoch (UTC) representation of when the object was/will be ready for deletion
//! IV. There is common class behaviors that are required in order to manage it using CRUD
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
//! use scaffolding_core::*;
//! use scaffolding_macros::*;
//!
//! #[as_entity]
//! #[derive(Debug, Clone, Scaffolding)]
//! struct MyEntity {
//!     b: bool,
//! }
//!
//! impl MyEntity {
//!     fn new(arg: bool) -> Self {
//!         Self {
//!             id: <Self as Scaffolding>::id(),
//!             created_dtm: <Self as Scaffolding>::now(),
//!             modified_dtm: <Self as Scaffolding>::now(),
//!             inactive_dtm: <Self as Scaffolding>::add_months(<Self as Scaffolding>::now(), 12),
//!             expired_dtm: <Self as Scaffolding>::add_years(<Self as Scaffolding>::now(), 3),
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

use chrono::{DateTime, Duration, Months, Utc};
use uuid::Uuid;

/// The core behavior of an Scaffolding object
pub trait Scaffolding {
    /// generates a uuid v4 value
    fn id() -> String {
        Uuid::new_v4().to_string()
    }

    /// adds x days to the timestamp
    fn add_days(dtm: i64, days: i64) -> i64 {
        let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Duration::try_days(days).unwrap();
        dt.timestamp()
    }

    /// adds x months to the timestamp
    fn add_months(dtm: i64, months: u32) -> i64 {
        let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Months::new(months);
        dt.timestamp()
    }

    /// adds x years to the timestamp
    fn add_years(dtm: i64, years: u32) -> i64 {
        let dt = DateTime::from_timestamp(dtm, 0).unwrap() + Months::new(years * 12);
        dt.timestamp()
    }

    /// provided the default unix epoch time (UTC) as seconds
    /// for the timestamp: 9999-12-31 23:59:59
    fn never() -> i64 {
        253402261199
    }

    /// generate the current unix epoch time (UTC) as seconds
    fn now() -> i64 {
        Utc::now().timestamp()
    }
}
