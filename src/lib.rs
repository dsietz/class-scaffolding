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
//!  + The `activity` is required and by default is an empty list
//!
//! ### Example
//! Add Scaffolding to a `struct` and `impl` using macros and defaults
//! ```rust
//! extern crate scaffolding_core;
//!
//! use scaffolding_core::{defaults, ActivityItem, Scaffolding};
//! use scaffolding_macros::*;
//! use serde_derive::{Deserialize, Serialize};
//! // Required for scaffolding metadata functionality
//! use std::collections::BTreeMap;
//!
//! #[scaffolding_struct("metadata")]
//! #[derive(Debug, Clone, Deserialize, Serialize, Scaffolding)]
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
//! assert_eq!(entity.id.len(), "54324f57-9e6b-4142-b68d-1d4c86572d0a".len());
//! assert_eq!(entity.created_dtm, defaults::now());
//! assert_eq!(entity.modified_dtm, defaults::now());
//! // becomes inactive in 90 days
//! assert_eq!(entity.inactive_dtm, defaults::add_days(defaults::now(), 90));
//! // expires in 3 years
//! assert_eq!(entity.expired_dtm, defaults::add_years(defaults::now(), 3));
//!
//! // add activity to the activty log
//! entity.log_activity("cancelled".to_string(), "The customer has cancelled their service".to_string());
//! assert_eq!(entity.get_activity("cancelled".to_string()).len(), 1);
//!
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
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use errors::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Supporting Classes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityItem {
    pub created_dtm: i64,
    pub action: String,
    pub description: String,
}

impl ActivityItem {
    pub fn new(name: String, descr: String) -> Self {
        Self {
            created_dtm: defaults::now(),
            action: name,
            description: descr,
        }
    }

    pub fn deserialized(serialized: &[u8]) -> Result<ActivityItem, DeserializeError> {
        match serde_json::from_slice(&serialized) {
            Ok(item) => Ok(item),
            Err(err) => {
                println!("{}", err);
                Err(DeserializeError)
            }
        }
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// The core behavior of an Scaffolding object
pub trait Scaffolding {
    /// This funciton adds a ActivityItem to the activity log
    fn log_activity(&mut self, name: String, descr: String);
    /// This function retrieves all the ActivityItems that have the specified action (name)
    fn get_activity(&self, name: String) -> Vec<ActivityItem>;
    /// This function instantiates an entity from a JSON string.
    ///
    /// #Example
    ///
    /// ```rust
    /// #[macro_use]
    /// // extern crate scaffolding_core;
    /// // extern crate scaffolding_macros;
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    ///
    /// #[scaffolding_struct]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding)]
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
    fn deserialized<T: DeserializeOwned>(serialized: &[u8]) -> Result<T, DeserializeError> {
        match serde_json::from_slice::<T>(&serialized) {
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
    /// // extern crate scaffolding_core;
    /// // extern crate scaffolding_macros;
    ///     
    /// use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    /// use scaffolding_macros::*;
    /// use serde_derive::{Deserialize, Serialize};
    ///
    /// #[scaffolding_struct]
    /// #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding)]
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
