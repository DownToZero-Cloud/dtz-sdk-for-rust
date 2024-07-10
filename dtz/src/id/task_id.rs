use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TaskId {
    pub id: Uuid,
}

impl Default for TaskId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("task-{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for TaskId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TaskIdVisitor;

        impl<'de> serde::de::Visitor<'de> for TaskIdVisitor {
            type Value = TaskId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with 'task-' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix("task-") {
                    let uuid = Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(TaskId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(TaskIdVisitor)
    }
}

impl serde::Serialize for TaskId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
