/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueCertificateRequest {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl IssueCertificateRequest {
    pub fn new() -> IssueCertificateRequest {
        IssueCertificateRequest {
            domain: None,
        }
    }
}


