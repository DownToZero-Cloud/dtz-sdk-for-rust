use std::fmt::Display;

static PREFIX: &str = "task-";

#[derive(Debug, Clone, PartialEq)]
pub struct TaskId {
    pub id: String,
}

impl Default for TaskId {
    fn default() -> Self {
        Self {
            id: crate::generate_internal_id(16),
        }
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl TryFrom<String> for TaskId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(TaskId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl TryFrom<&str> for TaskId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(TaskId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl<'de> serde::Deserialize<'de> for TaskId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TaskIdVisitor;

        impl serde::de::Visitor<'_> for TaskIdVisitor {
            type Value = TaskId;

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
                    Ok(TaskId {
                        id: id_str.to_string(),
                    })
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

#[test]
fn key_invalid_1() {
    let k = "abc-dsfdg";
    let task: Result<TaskId, String> = TaskId::try_from(k);
    assert!(task.is_err())
}

#[test]
fn key_valid_1() {
    let k = "task-dsfdg";
    let task: Result<TaskId, String> = TaskId::try_from(k);
    assert!(task.is_ok())
}

#[test]
fn key_valid_2() {
    let k = "task-0190c589-eb70-7980-97cf-af67b3a84116";
    let task: Result<TaskId, String> = TaskId::try_from(k);
    assert!(task.is_ok())
}

#[test]
fn key_invalid_2() {
    let k = "abc-0190c589-eb70-7980-97cf-af67b3a84116";
    let task: Result<TaskId, String> = TaskId::try_from(k);
    assert!(task.is_err())
}
