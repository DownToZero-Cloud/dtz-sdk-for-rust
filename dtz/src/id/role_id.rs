use std::fmt::Display;
use uuid::Uuid;

static PREFIX: &str = "role-";

#[derive(Debug)]
pub struct RoleId {
    pub id: Uuid,
}

impl Default for RoleId {
    fn default() -> Self {
        Self { id: Uuid::now_v7() }
    }
}

impl Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{PREFIX}{}", self.id))
    }
}

impl From<Uuid> for RoleId {
    fn from(id: Uuid) -> Self {
        RoleId { id }
    }
}