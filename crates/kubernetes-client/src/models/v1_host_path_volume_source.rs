/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1HostPathVolumeSource : Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1HostPathVolumeSource {
    /// Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(rename = "path")]
    pub path: String,
    /// Type for HostPath Volume Defaults to \"\" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl V1HostPathVolumeSource {
    /// Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.
    pub fn new(path: String) -> V1HostPathVolumeSource {
        V1HostPathVolumeSource {
            path,
            _type: None,
        }
    }
}

