static PREFIX: &str = "object-";

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ObjectId {
    pub id: uuid::Uuid,
}

impl Default for ObjectId {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
        }
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ObjectIdVisitor;

        impl<'de> serde::de::Visitor<'de> for ObjectIdVisitor {
            type Value = ObjectId;

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
                    let uuid = uuid::Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(ObjectId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(ObjectIdVisitor)
    }
}

impl serde::Serialize for ObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<uuid::Uuid> for ObjectId {
    fn from(id: uuid::Uuid) -> Self {
        ObjectId { id }
    }
}

#[test]
fn test_from() {
    let uuid = uuid::Uuid::now_v7();
    let ctx = ObjectId::from(uuid);
    assert_eq!(ctx.id, uuid);
}
