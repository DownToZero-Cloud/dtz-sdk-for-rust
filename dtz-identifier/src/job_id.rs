use std::fmt::Display;
use uuid::Uuid;

static PREFIX: &str = "job-";

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct JobId {
    pub id: Uuid,
}

impl Default for JobId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for JobId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct JobIdVisitor;

        impl<'de> serde::de::Visitor<'de> for JobIdVisitor {
            type Value = JobId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with 'job-' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix(PREFIX) {
                    let uuid = Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(JobId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(JobIdVisitor)
    }
}

impl serde::Serialize for JobId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<Uuid> for JobId {
    fn from(id: Uuid) -> Self {
        JobId { id }
    }
}
