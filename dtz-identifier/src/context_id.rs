use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::Display;

static PREFIX: &str = "context-";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextId {
    pub id: String,
}

impl ContextId {
    pub fn nil() -> Self {
        Self {
            id: "00000000".to_string(),
        }
    }
}

impl Default for ContextId {
    fn default() -> Self {
        Self {
            id: crate::generate_internal_id(8),
        }
    }
}

impl Display for ContextId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl TryFrom<String> for ContextId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(ContextId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl TryFrom<&str> for ContextId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(ContextId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl<'de> Deserialize<'de> for ContextId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContextIdVisitor;

        impl Visitor<'_> for ContextIdVisitor {
            type Value = ContextId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with '")?;
                formatter.write_str(PREFIX)?;
                formatter.write_str("' followed by a 8-character alphanumeric string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(id_str) = value.strip_prefix(PREFIX) {
                    Ok(ContextId {
                        id: id_str.to_string(),
                    })
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

#[test]
fn test_from() {
    let id = crate::generate_internal_id(8);
    let id_str = format!("{PREFIX}{id}");
    let ctx = ContextId::try_from(id_str).unwrap();
    assert_eq!(ctx.id, id);
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
    assert!(apikey.is_ok())
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

#[test]
fn test_nil() {
    let ctx = ContextId::nil();
    assert_eq!(ctx.id, "00000000");
}

