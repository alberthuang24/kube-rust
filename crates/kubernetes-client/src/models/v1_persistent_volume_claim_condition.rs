/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PersistentVolumeClaimCondition : PersistentVolumeClaimCondition contails details about state of pvc



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1PersistentVolumeClaimCondition {
    /// Last time we probed the condition.
    #[serde(rename = "lastProbeTime", skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<String>,
    /// Last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports \"ResizeStarted\" that means the underlying persistent volume is being resized.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1PersistentVolumeClaimCondition {
    /// PersistentVolumeClaimCondition contails details about state of pvc
    pub fn new(status: String, _type: String) -> V1PersistentVolumeClaimCondition {
        V1PersistentVolumeClaimCondition {
            last_probe_time: None,
            last_transition_time: None,
            message: None,
            reason: None,
            status,
            _type,
        }
    }
}


