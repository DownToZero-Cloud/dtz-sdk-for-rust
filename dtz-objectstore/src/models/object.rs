/*
 * DTZ Objectstore
 *
 * a generated client for the DTZ Objectstore API
 *
 * The version of the OpenAPI document: 1.0.4
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Object {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            id: None,
            name: None,
        }
    }
}


