extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use scaffolding_core::{defaults, Scaffolding};
    use scaffolding_macros::*;

    #[scaffolding_entity]
    #[derive(Debug, Clone, Scaffolding)]
    struct MyEntity {
        b: bool,
        n: i64,
    }

    impl MyEntity {
        fn new(arg: bool) -> Self {
            Self {
                id: defaults::id(),
                created_dtm: defaults::now(),
                modified_dtm: defaults::now(),
                inactive_dtm: defaults::add_days(defaults::now(), 90),
                expired_dtm: defaults::add_years(defaults::now(), 3),
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
        let entity = MyEntity::new(true);

        // scaffolding attributes
        assert_eq!(
            entity.id.len(),
            "54324f57-9e6b-4142-b68d-1d4c86572d0a".len()
        );
        assert_eq!(entity.created_dtm, now);
        assert_eq!(entity.modified_dtm, now);
        assert_eq!((entity.inactive_dtm - entity.modified_dtm) / 86400, 90);
        assert_eq!((entity.expired_dtm - entity.modified_dtm) / 86400, 1095);

        // extended attributes
        assert_eq!(entity.b, true);
        assert_eq!(entity.n, never);

        // extended behavior
        assert_eq!(entity.my_func(), "my function");
    }
}
