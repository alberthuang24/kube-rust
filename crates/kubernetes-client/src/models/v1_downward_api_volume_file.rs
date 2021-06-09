/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1DownwardApiVolumeFile : DownwardAPIVolumeFile represents information to create the file containing the pod field



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1DownwardApiVolumeFile {
    #[serde(rename = "fieldRef", skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<crate::models::V1ObjectFieldSelector>,
    /// Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "resourceFieldRef", skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<crate::models::V1ResourceFieldSelector>,
}

impl V1DownwardApiVolumeFile {
    /// DownwardAPIVolumeFile represents information to create the file containing the pod field
    pub fn new(path: String) -> V1DownwardApiVolumeFile {
        V1DownwardApiVolumeFile {
            field_ref: None,
            mode: None,
            path,
            resource_field_ref: None,
        }
    }
}


