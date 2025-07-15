use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt::Display;

static PREFIX: &str = "identity-";

#[derive(Debug, Clone, PartialEq)]
pub struct IdentityId {
    pub id: String,
}

impl IdentityId {
    pub fn nil() -> Self {
        Self {
            id: "00000000".to_string(),
        }
    }
}

impl Display for IdentityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl Default for IdentityId {
    fn default() -> Self {
        Self {
            id: crate::generate_internal_id(8),
        }
    }
}

impl<'de> Deserialize<'de> for IdentityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdentityIdVisitor;

        impl Visitor<'_> for IdentityIdVisitor {
            type Value = IdentityId;

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
                    Ok(IdentityId {
                        id: id_str.to_string(),
                    })
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

impl TryFrom<String> for IdentityId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(IdentityId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl TryFrom<&str> for IdentityId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(IdentityId {
                id: id_str.to_string(),
            })
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

#[test]
fn key_invalid_1() {
    let k = "abc-dsfdg";
    let identity: Result<IdentityId, String> = IdentityId::try_from(k);
    assert!(identity.is_err())
}

#[test]
fn key_valid_1() {
    let k = "identity-dsfdg";
    let identity: Result<IdentityId, String> = IdentityId::try_from(k);
    assert!(identity.is_ok())
}

#[test]
fn key_valid_2() {
    let k = "identity-0190c589-eb70-7980-97cf-af67b3a84116";
    let identity: Result<IdentityId, String> = IdentityId::try_from(k);
    assert!(identity.is_ok())
}

#[test]
fn key_invalid_2() {
    let k = "abc-0190c589-eb70-7980-97cf-af67b3a84116";
    let identity: Result<IdentityId, String> = IdentityId::try_from(k);
    assert!(identity.is_err())
}

#[test]
fn key_null() {
    let k = "identity-null";
    let identity: Result<IdentityId, String> = IdentityId::try_from(k);
    assert!(identity.is_ok())
}

#[test]
fn key_nil() {
    let k = "identity-00000000";
    let identity: Result<IdentityId, String> = IdentityId::try_from(k);
    assert!(identity.is_ok())
}


#[test]
fn key_nil_2() {
    let k = "identity-00000000";
    let identity: Result<IdentityId, String> = IdentityId::try_from(k);
    let nil_id = IdentityId::nil();
    assert_eq!(identity.unwrap(), nil_id);
}


