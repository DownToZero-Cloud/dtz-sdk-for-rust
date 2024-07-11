/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.0.9
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Login {
    #[serde(rename = "providerName")]
    pub provider_name: String,
}

impl Login {
    pub fn new(provider_name: String) -> Login {
        Login {
            provider_name,
        }
    }
}


