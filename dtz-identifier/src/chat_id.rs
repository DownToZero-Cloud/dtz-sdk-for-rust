static PREFIX: &str = "chat-";

#[derive(Debug, Clone, PartialEq)]
pub struct ChatId {
    pub id: String,
}

impl Default for ChatId {
    fn default() -> Self {
        Self {
            id: crate::generate_internal_id(8),
        }
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl TryFrom<String> for ChatId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(ChatId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl TryFrom<&str> for ChatId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(id_str) = value.strip_prefix(PREFIX) {
            Ok(ChatId {
                id: id_str.to_string(),
            })
        } else {
            Err("invalid format".to_string())
        }
    }
}

impl<'de> serde::Deserialize<'de> for ChatId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ChatIdVisitor;

        impl serde::de::Visitor<'_> for ChatIdVisitor {
            type Value = ChatId;

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
                    Ok(ChatId {
                        id: id_str.to_string(),
                    })
                } else {
                    Err(E::custom("invalid format"))
                }
            }
        }

        deserializer.deserialize_str(ChatIdVisitor)
    }
}

impl serde::Serialize for ChatId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[test]
fn test_from() {
    let ctx = ChatId::try_from("chat-12345678").unwrap();
    assert_eq!(ctx.id, "12345678");
}
