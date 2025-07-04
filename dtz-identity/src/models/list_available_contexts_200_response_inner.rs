/*
 * DTZ Identity
 *
 * a generated client for the DTZ Identity API
 *
 * The version of the OpenAPI document: 2.0.1
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use crate::models;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAvailableContexts200ResponseInner {
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<dtz_identifier::ContextId>,
}

impl ListAvailableContexts200ResponseInner {
    pub fn new() -> ListAvailableContexts200ResponseInner {
        ListAvailableContexts200ResponseInner {
            context_id: None,
        }
    }
}

