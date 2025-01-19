use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::Display;
use uuid::Uuid;

static PREFIX: &str = "context-";

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct ContextId {
    pub id: Uuid,
}

impl Default for ContextId {
    fn default() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl Display for ContextId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> Deserialize<'de> for ContextId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContextIdVisitor;

        impl<'de> Visitor<'de> for ContextIdVisitor {
            type Value = ContextId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with 'context-' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix(PREFIX) {
                    let uuid = Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(ContextId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(ContextIdVisitor)
    }
}

impl Serialize for ContextId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<Uuid> for ContextId {
    fn from(id: Uuid) -> Self {
        ContextId { id }
    }
}

#[test]
fn test_from() {
    let uuid = Uuid::new_v4();
    let ctx = ContextId::from(uuid);
    assert_eq!(ctx.id, uuid);
}

impl TryFrom<&str> for ContextId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(uuid_str) = value.strip_prefix(PREFIX) {
            let uuid =
                uuid::Uuid::parse_str(uuid_str).map_err(|_e| "invalid format".to_string())?;
            Ok(ContextId { id: uuid })
        } else {
            Err("invalid format".to_string())
        }
    }
}

#[test]
fn key_invalid_1() {
    let k = "abc-dsfdg";
    let apikey: Result<ContextId, String> = ContextId::try_from(k);
    assert!(apikey.is_err())
}

#[test]
fn key_invalid_2() {
    let k = "context-dsfdg";
    let apikey: Result<ContextId, String> = ContextId::try_from(k);
    assert!(apikey.is_err())
}

#[test]
fn key_valid_1() {
    let k = "context-0190c589-eb70-7980-97cf-af67b3a84116";
    let apikey: Result<ContextId, String> = ContextId::try_from(k);
    assert!(apikey.is_ok())
}

#[test]
fn key_invalid_3() {
    let k = "abc-0190c589-eb70-7980-97cf-af67b3a84116";
    let apikey: Result<ContextId, String> = ContextId::try_from(k);
    assert!(apikey.is_err())
}
