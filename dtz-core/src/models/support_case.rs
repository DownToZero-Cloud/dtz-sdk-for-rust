/*
 * DTZ Core Api
 *
 * a generated client for the DTZ Core API
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused)]
use crate::models;




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportCase {
    #[serde(rename = "caseId", skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "timeline", skip_serializing_if = "Option::is_none")]
    pub timeline: Option<Vec<models::SupportCaseTimelineInner>>,
}

impl SupportCase {
    pub fn new() -> SupportCase {
        SupportCase {
            case_id: None,
            created: None,
            status: None,
            timeline: None,
        }
    }
}


