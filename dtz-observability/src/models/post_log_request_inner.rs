/*
 * DTZ Observability
 *
 * a generated client for the DTZ Observability API
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: jens@apimeister.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use crate::models;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostLogRequestInner {
    #[serde(rename = "tsNanos", skip_serializing_if = "Option::is_none")]
    pub ts_nanos: Option<String>,
    #[serde(rename = "spanId", skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    #[serde(rename = "traceId", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "childId", skip_serializing_if = "Option::is_none")]
    pub child_id: Option<String>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<models::PostLogRequestInnerLinksInner>>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<models::PostLogRequestInnerAttachmentsInner>>,
}

impl PostLogRequestInner {
    pub fn new() -> PostLogRequestInner {
        PostLogRequestInner {
            ts_nanos: None,
            span_id: None,
            trace_id: None,
            parent_id: None,
            child_id: None,
            payload: None,
            attributes: None,
            tags: None,
            links: None,
            attachments: None,
        }
    }
}

