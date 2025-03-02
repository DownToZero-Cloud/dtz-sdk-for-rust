static PREFIX: &str = "feed-";

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FeedId {
    pub id: uuid::Uuid,
}

impl Default for FeedId {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
        }
    }
}

impl std::fmt::Display for FeedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for FeedId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FeedIdVisitor;

        impl serde::de::Visitor<'_> for FeedIdVisitor {
            type Value = FeedId;

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
                    Ok(FeedId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(FeedIdVisitor)
    }
}

impl serde::Serialize for FeedId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<uuid::Uuid> for FeedId {
    fn from(id: uuid::Uuid) -> Self {
        FeedId { id }
    }
}

#[test]
fn test_from() {
    let uuid = uuid::Uuid::now_v7();
    let ctx = FeedId::from(uuid);
    assert_eq!(ctx.id, uuid);
}
