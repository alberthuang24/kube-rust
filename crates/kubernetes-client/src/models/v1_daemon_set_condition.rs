/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1DaemonSetCondition : DaemonSetCondition describes the state of a DaemonSet at a certain point.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1DaemonSetCondition {
    /// Last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    #[serde(rename = "status")]
    pub status: String,
    /// Type of DaemonSet condition.
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1DaemonSetCondition {
    /// DaemonSetCondition describes the state of a DaemonSet at a certain point.
    pub fn new(status: String, _type: String) -> V1DaemonSetCondition {
        V1DaemonSetCondition {
            last_transition_time: None,
            message: None,
            reason: None,
            status,
            _type,
        }
    }
}


