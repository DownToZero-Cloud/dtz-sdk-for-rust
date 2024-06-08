/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.7
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "roleId")]
    pub role_id: uuid::Uuid,
    #[serde(rename = "roleAlias")]
    pub role_alias: String,
    #[serde(rename = "roleScope")]
    pub role_scope: String,
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<uuid::Uuid>,
    #[serde(rename = "exposure")]
    pub exposure: String,
    #[serde(rename = "assignedToUser", skip_serializing_if = "Option::is_none")]
    pub assigned_to_user: Option<bool>,
}

impl Role {
    pub fn new(role_id: uuid::Uuid, role_alias: String, role_scope: String, exposure: String) -> Role {
        Role {
            role_id,
            role_alias,
            role_scope,
            context_id: None,
            exposure,
            assigned_to_user: None,
        }
    }
}


