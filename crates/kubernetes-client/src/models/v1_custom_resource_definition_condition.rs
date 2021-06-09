/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CustomResourceDefinitionCondition : CustomResourceDefinitionCondition contains details for the current condition of this pod.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CustomResourceDefinitionCondition {
    /// lastTransitionTime last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// message is a human-readable message indicating details about last transition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// reason is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// status is the status of the condition. Can be True, False, Unknown.
    #[serde(rename = "status")]
    pub status: String,
    /// type is the type of the condition. Types include Established, NamesAccepted and Terminating.
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1CustomResourceDefinitionCondition {
    /// CustomResourceDefinitionCondition contains details for the current condition of this pod.
    pub fn new(status: String, _type: String) -> V1CustomResourceDefinitionCondition {
        V1CustomResourceDefinitionCondition {
            last_transition_time: None,
            message: None,
            reason: None,
            status,
            _type,
        }
    }
}


