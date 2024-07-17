/*
 * DTZ Objectstore
 *
 * a generated client for the DTZ Objectstore API
 *
 * The version of the OpenAPI document: 1.0.5
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "amountOfFiles", skip_serializing_if = "Option::is_none")]
    pub amount_of_files: Option<f64>,
    #[serde(rename = "amountOfStorage", skip_serializing_if = "Option::is_none")]
    pub amount_of_storage: Option<f64>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            amount_of_files: None,
            amount_of_storage: None,
        }
    }
}


