//! The `scaffolding` module provides the abstract class for developing object oriented classes.
//! 
//! 
//! 
//! 

extern crate uuid;

use uuid::Uuid;

pub struct AbstractEntity {
    pub uid: String,
}

impl AbstractEntity {
    pub fn new() -> Self {
        AbstractEntity {
            uid: Uuid::new_v4().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_abstract_entity() -> AbstractEntity {
        let entity = AbstractEntity::new();

        entity
    }

    #[test]
    fn test_entity_id_ok() {
        let entity = get_abstract_entity();
        println!("{}", entity.uid);
        assert_eq!(entity.uid.len(), 36);
    }
}
