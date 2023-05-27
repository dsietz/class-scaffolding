//! The `scaffolding` module provides the abstract class for developing object oriented classes.
//! 
//! 
//! 
//! 

extern crate uuid;

use uuid::Uuid;

// https://users.rust-lang.org/t/struct-inheritance-embedding-best-practice/10627/5 
// https://doc.rust-lang.org/reference/attributes/derive.html
// https://stackoverflow.com/questions/53135923/how-to-write-a-custom-derive-macro
// https://docs.rs/syn/latest/syn/index.html#example-of-a-custom-derive

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
