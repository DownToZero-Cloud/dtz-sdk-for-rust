/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.20
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignupRequest {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl SignupRequest {
    pub fn new(email: String, password: String) -> SignupRequest {
        SignupRequest {
            email,
            password,
        }
    }
}


