/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.10
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename = "contextId")]
    pub context_id: dtz_identifier::ContextId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "created")]
    pub created: String,
}

impl Domain {
    pub fn new(context_id: dtz_identifier::ContextId, name: String, verified: bool, created: String) -> Domain {
        Domain {
            context_id,
            name,
            verified,
            created,
        }
    }
}


