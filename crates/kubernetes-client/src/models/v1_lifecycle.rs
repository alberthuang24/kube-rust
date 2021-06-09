/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1Lifecycle : Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1Lifecycle {
    #[serde(rename = "postStart", skip_serializing_if = "Option::is_none")]
    pub post_start: Option<crate::models::V1Handler>,
    #[serde(rename = "preStop", skip_serializing_if = "Option::is_none")]
    pub pre_stop: Option<crate::models::V1Handler>,
}

impl V1Lifecycle {
    /// Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.
    pub fn new() -> V1Lifecycle {
        V1Lifecycle {
            post_start: None,
            pre_stop: None,
        }
    }
}


