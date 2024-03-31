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
//! use scaffolding_core::{defaults, ActivityItem, Note, Scaffolding, ScaffoldingNotes, ScaffoldingTags};
//! use scaffolding_macros::*;
//! use serde_derive::{Deserialize, Serialize};
//! // Required for scaffolding metadata functionality
//! use std::collections::BTreeMap;
//!
//! // (1) Define the structure - Required
//! #[scaffolding_struct("metadata","notes","tags")]
//! #[derive(Debug, Clone, Deserialize, Serialize, Scaffolding, ScaffoldingNotes, ScaffoldingTags)]
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
//!     #[scaffolding_fn("metadata","notes","tags")]
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
//! /* use the notes functionality */
//! // (1) Insert a note
//! let note_id = entity.insert_note(
//!   "fsmith".to_string(),
//!   "This was updated".as_bytes().to_vec(),
//!   None,
//! );
//! // (2) Modify the note
//! entity.modify_note(
//!   note_id.clone(),
//!   "fsmith".to_string(),
//!   "This was updated again".as_bytes().to_vec(),
//!   Some("private".to_string()),
//! );
//! // (3) Read the note's content
//! let read_note = entity.get_note(note_id.clone()).unwrap().content_as_string().unwrap();
//! println!("{}", read_note);
//! // (4) Search for notes that contain the word `updated`
//! let search_results = entity.search_notes("updated".to_string());
//! assert_eq!(search_results.len(), 1);
//! // (5) Delete the note
//! entity.remove_note(note_id);
//!
//! /* use the metadata functionality */
//! entity.metadata.insert("field_1".to_string(), "myvalue".to_string());
//! assert_eq!(entity.metadata.len(), 1);
//!
//! // manage tags
//! entity.add_tag("tag_1".to_string());
//! entity.add_tag("tag_2".to_string());
//! entity.add_tag("tag_3".to_string());
//! assert!(entity.has_tag("tag_1".to_string()));
//! entity.remove_tag("tag_2".to_string());
//! assert_eq!(entity.tags.len(), 2);
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
/// The core behavior of a Scaffolding object
pub trait Scaffolding {
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

/// The notes behavior of a Scaffolding object
pub trait ScaffoldingNotes {
    fn get_note(&self, id: String) -> Option<&Note>;
    fn insert_note(&mut self, auth: String, cont: Vec<u8>, acc: Option<String>) -> String;
    fn modify_note(&mut self, id: String, auth: String, cont: Vec<u8>, acc: Option<String>);
    fn search_notes(&mut self, search: String) -> Vec<Note>;
    fn remove_note(&mut self, id: String);
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
