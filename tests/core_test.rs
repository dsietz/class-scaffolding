#[macro_use]
extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    use scaffolding_macros::*;
    use serde_derive::Deserialize;

    #[scaffolding_struct]
    #[derive(Debug, Clone, Scaffolding)]
    struct MyEntity {
        b: bool,
        n: i64,
    }

    impl MyEntity {
        #[scaffolding_fn]
        fn new(arg: bool) -> Self {
            Self {
                b: arg,
                n: defaults::never(),
            }
        }

        fn my_func(&self) -> String {
            "my function".to_string()
        }
    }

    #[test]
    fn test_entity_new() {
        let now = Utc::now().timestamp();
        let never = 253402261199;
        let mut entity = MyEntity::new(true);

        // scaffolding attributes
        assert_eq!(
            entity.id.len(),
            "54324f57-9e6b-4142-b68d-1d4c86572d0a".len()
        );
        assert_eq!(entity.created_dtm, now);
        assert_eq!(entity.modified_dtm, now);
        assert_eq!((entity.inactive_dtm - entity.modified_dtm) / 86400, 90);
        assert_eq!((entity.expired_dtm - entity.modified_dtm) / 86400, 1095);

        // scaffolding behavior
        entity.log_activity(
            "updated".to_string(),
            "The object has been updated".to_string(),
        );
        entity.log_activity(
            "updated".to_string(),
            "The object has been updated".to_string(),
        );
        entity.log_activity(
            "cancelled".to_string(),
            "The object has been cancelled".to_string(),
        );

        assert_eq!(entity.activity.len(), 3);
        assert_eq!(entity.get_activity("updated".to_string()).len(), 2);

        // extended attributes
        assert_eq!(entity.b, true);
        assert_eq!(entity.n, never);

        // extended behavior
        assert_eq!(entity.my_func(), "my function");
    }
}
