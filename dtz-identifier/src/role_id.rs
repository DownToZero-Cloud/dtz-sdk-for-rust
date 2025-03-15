use std::fmt::Display;

static PREFIX: &str = "role-";

#[derive(Debug, Clone, PartialEq)]
pub struct RoleId {
    pub id: String,
}

impl Default for RoleId {
    fn default() -> Self {
        Self {
            id: crate::generate_internal_id(8),
        }
    }
}

impl Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl TryFrom<String> for RoleId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(RoleId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl TryFrom<&str> for RoleId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(RoleId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl<'de> serde::Deserialize<'de> for RoleId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RoleIdVisitor;

        impl serde::de::Visitor<'_> for RoleIdVisitor {
            type Value = RoleId;

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
                    Ok(RoleId {
                        id: id_str.to_string(),
                    })
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

#[test]
fn key_invalid_1() {
    let k = "abc-dsfdg";
    let role: Result<RoleId, String> = RoleId::try_from(k);
    assert!(role.is_err())
}

#[test]
fn key_valid_1() {
    let k = "role-dsfdg";
    let role: Result<RoleId, String> = RoleId::try_from(k);
    assert!(role.is_ok())
}

#[test]
fn key_valid_2() {
    let k = "role-0190c589-eb70-7980-97cf-af67b3a84116";
    let role: Result<RoleId, String> = RoleId::try_from(k);
    assert!(role.is_ok())
}

#[test]
fn key_invalid_2() {
    let k = "abc-0190c589-eb70-7980-97cf-af67b3a84116";
    let role: Result<RoleId, String> = RoleId::try_from(k);
    assert!(role.is_err())
}
