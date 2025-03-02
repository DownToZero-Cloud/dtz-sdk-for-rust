use std::fmt::Display;
use uuid::Uuid;

static PREFIX: &str = "service-";

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ServiceId {
    pub id: Uuid,
}

impl Default for ServiceId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for ServiceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ServiceIdVisitor;

        impl serde::de::Visitor<'_> for ServiceIdVisitor {
            type Value = ServiceId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with 'service-' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix(PREFIX) {
                    let uuid = Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(ServiceId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(ServiceIdVisitor)
    }
}

impl serde::Serialize for ServiceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<Uuid> for ServiceId {
    fn from(id: Uuid) -> Self {
        ServiceId { id }
    }
}
