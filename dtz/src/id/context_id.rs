use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ContextId {
    pub id: Uuid,
}

impl Default for ContextId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl Display for ContextId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("context-{}", self.id))
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
                if let Some(uuid_str) = value.strip_prefix("context-") {
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
