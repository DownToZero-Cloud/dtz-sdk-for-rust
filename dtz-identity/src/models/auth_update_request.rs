/*
 * DTZ Identity
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthUpdateRequest {
    #[serde(rename = "authentication_id", skip_serializing_if = "Option::is_none")]
    pub authentication_id: Option<String>,
}

impl AuthUpdateRequest {
    pub fn new() -> AuthUpdateRequest {
        AuthUpdateRequest {
            authentication_id: None,
        }
    }
}


