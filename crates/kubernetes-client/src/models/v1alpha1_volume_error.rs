/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1VolumeError : VolumeError captures an error encountered during a volume operation.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1alpha1VolumeError {
    /// String detailing the error encountered during Attach or Detach operation. This string maybe logged, so it should not contain sensitive information.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Time the error was encountered.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl V1alpha1VolumeError {
    /// VolumeError captures an error encountered during a volume operation.
    pub fn new() -> V1alpha1VolumeError {
        V1alpha1VolumeError {
            message: None,
            time: None,
        }
    }
}


