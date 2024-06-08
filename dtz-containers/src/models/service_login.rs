/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * The version of the OpenAPI document: 1.0.5
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLogin {
    #[serde(rename = "providerName")]
    pub provider_name: String,
}

impl ServiceLogin {
    pub fn new(provider_name: String) -> ServiceLogin {
        ServiceLogin {
            provider_name,
        }
    }
}


