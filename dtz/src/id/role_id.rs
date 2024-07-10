use std::fmt::Display;

use uuid::Uuid;

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
        f.write_str(&format!("role-{}", self.id))
    }
}
