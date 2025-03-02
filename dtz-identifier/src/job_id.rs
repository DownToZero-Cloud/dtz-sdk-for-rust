use std::fmt::Display;

static PREFIX: &str = "job-";

#[derive(Debug, Clone, PartialEq)]
pub struct JobId {
    pub id: String,
}

impl Default for JobId {
    fn default() -> Self {
        Self {
            id: crate::generate_internal_id(),
        }
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl TryFrom<String> for JobId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(JobId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl TryFrom<&str> for JobId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(JobId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl<'de> serde::Deserialize<'de> for JobId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct JobIdVisitor;

        impl serde::de::Visitor<'_> for JobIdVisitor {
            type Value = JobId;

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
                    Ok(JobId {
                        id: id_str.to_string(),
                    })
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

#[test]
fn key_invalid_1() {
    let k = "abc-dsfdg";
    let job: Result<JobId, String> = JobId::try_from(k);
    assert!(job.is_err())
}

#[test]
fn key_valid_1() {
    let k = "job-dsfdg";
    let job: Result<JobId, String> = JobId::try_from(k);
    assert!(job.is_ok())
}

#[test]
fn key_valid_2() {
    let k = "job-0190c589-eb70-7980-97cf-af67b3a84116";
    let job: Result<JobId, String> = JobId::try_from(k);
    assert!(job.is_ok())
}

#[test]
fn key_invalid_2() {
    let k = "abc-0190c589-eb70-7980-97cf-af67b3a84116";
    let job: Result<JobId, String> = JobId::try_from(k);
    assert!(job.is_err())
}
