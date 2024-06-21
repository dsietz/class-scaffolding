extern crate scaffolding_core;
extern crate scaffolding_macros;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use scaffolding_core::{defaults, ActivityItem, Scaffolding};
    use scaffolding_macros::*;
    use serde_derive::{Deserialize, Serialize};

    #[scaffolding_struct]
    #[derive(Clone, Debug, Deserialize, Serialize, Scaffolding)]
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

    #[test]
    fn test_entity_activity() {
        let mut entity = MyEntity::new(true);

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
    }

    #[test]
    fn test_entity_deserialize() {
        let never = 253402261199;
        let json = r#"{
            "b":true,
            "n":253402261199,
            "id":"b4d6c6db-7468-400a-8536-a5e83b1f2bdc",
            "created_dtm":1711802687,
            "modified_dtm":1711802687,
            "inactive_dtm":1719578687,
            "expired_dtm":1806410687,
            "activity":[
                {
                    "created_dtm":1711802687,
                    "action":"updated",
                    "description":"The object has been updated"
                },
                {
                    "created_dtm":1711802687,
                    "action":"updated",
                    "description":"The object has been updated"
                },
                {
                    "created_dtm":1711802687,
                    "action":"cancelled",
                    "description":"The object has been cancelled"
                }
                ]
            }"#;
        let deserialized = MyEntity::deserialized(json.as_bytes()).unwrap();
        assert_eq!(deserialized.id, "b4d6c6db-7468-400a-8536-a5e83b1f2bdc");
        assert_eq!(deserialized.activity.len(), 3);
        assert_eq!(deserialized.b, true);
        assert_eq!(deserialized.n, never);
        assert_eq!(deserialized.my_func(), "my function");
    }

    #[test]
    #[ignore]
    fn test_entity_serialize() {
        let mut entity = MyEntity::new(true);
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

        let expected = r#"{
            "b":true,
            "n":253402261199,
            "id":"b4d6c6db-7468-400a-8536-a5e83b1f2bdc",
            "created_dtm":1711802687,
            "modified_dtm":1711802687,
            "inactive_dtm":1719578687,
            "expired_dtm":1806410687,
            "activity":[
                {
                    "created_dtm":1711802687,
                    "action":"updated",
                    "description":"The object has been updated"
                },
                {
                    "created_dtm":1711802687,
                    "action":"updated",
                    "description":"The object has been updated"
                },
                {
                    "created_dtm":1711802687,
                    "action":"cancelled",
                    "description":"The object has been cancelled"
                }
                ]
            }"#;

        assert_eq!(entity.serialize(), expected);
    }
}
