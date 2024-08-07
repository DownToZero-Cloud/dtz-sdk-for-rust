use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::Display;
use uuid::Uuid;

static PREFIX: &str = "identity-";

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct IdentityId {
    pub id: Uuid,
}

impl Display for IdentityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl Default for IdentityId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl<'de> Deserialize<'de> for IdentityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdentityIdVisitor;

        impl<'de> Visitor<'de> for IdentityIdVisitor {
            type Value = IdentityId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with 'identity-' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix(PREFIX) {
                    let uuid = Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(IdentityId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(IdentityIdVisitor)
    }
}

impl Serialize for IdentityId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<Uuid> for IdentityId {
    fn from(id: Uuid) -> Self {
        IdentityId { id }
    }
}

impl TryFrom<&str> for IdentityId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(uuid_str) = value.strip_prefix(PREFIX) {
            let uuid =
                uuid::Uuid::parse_str(uuid_str).map_err(|_e| "invalid format".to_string())?;
            Ok(IdentityId { id: uuid })
        } else {
            Err("invalid format".to_string())
        }
    }
}

#[test]
fn id_tostring() {
    let id = IdentityId::default();
    let full_id = format!("{id}");
    println!("full-id: {full_id}");
    assert!(full_id.starts_with(PREFIX));
}
