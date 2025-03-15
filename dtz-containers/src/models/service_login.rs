/*
 * DTZ Containers
 *
 * a generated client for the DTZ Containers API
 *
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
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


