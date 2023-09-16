/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "status")]
    pub status: String,
}

impl ErrorResponse {
    pub fn new(status: String) -> ErrorResponse {
        ErrorResponse {
            status,
        }
    }
}


