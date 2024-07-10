static PREFIX: &str = "apikey-";

#[derive(Debug, Clone, PartialEq)]
pub struct ApiKeyId {
    pub id: uuid::Uuid,
}

impl std::fmt::Display for ApiKeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl<'de> serde::Deserialize<'de> for ApiKeyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ApiKeyIdVisitor;

        impl<'de> serde::de::Visitor<'de> for ApiKeyIdVisitor {
            type Value = ApiKeyId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string starting with 'apikey-' followed by a UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(uuid_str) = value.strip_prefix(PREFIX) {
                    let uuid = uuid::Uuid::parse_str(uuid_str).map_err(E::custom)?;
                    Ok(ApiKeyId { id: uuid })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(ApiKeyIdVisitor)
    }
}

impl serde::Serialize for ApiKeyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<uuid::Uuid> for ApiKeyId {
    fn from(id: uuid::Uuid) -> Self {
        ApiKeyId { id }
    }
}