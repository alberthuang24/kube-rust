/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1PersistentVolumeStatus : PersistentVolumeStatus is the current status of a persistent volume.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1PersistentVolumeStatus {
    /// A human-readable message indicating details about why the volume is in this state.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl V1PersistentVolumeStatus {
    /// PersistentVolumeStatus is the current status of a persistent volume.
    pub fn new() -> V1PersistentVolumeStatus {
        V1PersistentVolumeStatus {
            message: None,
            phase: None,
            reason: None,
        }
    }
}

