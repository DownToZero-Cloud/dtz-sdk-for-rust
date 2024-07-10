use std::fmt::Display;
use uuid::Uuid;

static PREFIX: &str = "execution-";

#[derive(Debug, Clone, Copy)]
pub struct ExecutionId {
    pub id: Uuid,
}

impl Default for ExecutionId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl Display for ExecutionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for ExecutionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ExecutionIdVisitor;

        impl<'de> serde::de::Visitor<'de> for ExecutionIdVisitor {
            type Value = ExecutionId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with '")?;
                formatter.write_str(PREFIX)?;
                formatter.write_str("' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix(PREFIX) {
                    let uuid = Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(ExecutionId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(ExecutionIdVisitor)
    }
}

impl serde::Serialize for ExecutionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<Uuid> for ExecutionId {
    fn from(id: Uuid) -> Self {
        ExecutionId { id }
    }
}