/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1FlowSchemaCondition : FlowSchemaCondition describes conditions for a FlowSchema.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1FlowSchemaCondition {
    /// `lastTransitionTime` is the last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// `message` is a human-readable message indicating details about last transition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// `reason` is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// `status` is the status of the condition. Can be True, False, Unknown. Required.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// `type` is the type of the condition. Required.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl V1beta1FlowSchemaCondition {
    /// FlowSchemaCondition describes conditions for a FlowSchema.
    pub fn new() -> V1beta1FlowSchemaCondition {
        V1beta1FlowSchemaCondition {
            last_transition_time: None,
            message: None,
            reason: None,
            status: None,
            _type: None,
        }
    }
}


