use std::fmt::Display;
use uuid::Uuid;

static PREFIX: &str = "role-";

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct RoleId {
    pub id: Uuid,
}

impl Default for RoleId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for RoleId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RoleIdVisitor;

        impl<'de> serde::de::Visitor<'de> for RoleIdVisitor {
            type Value = RoleId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with 'role-' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix(PREFIX) {
                    let uuid = Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(RoleId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(RoleIdVisitor)
    }
}

impl serde::Serialize for RoleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<Uuid> for RoleId {
    fn from(id: Uuid) -> Self {
        RoleId { id }
    }
}
