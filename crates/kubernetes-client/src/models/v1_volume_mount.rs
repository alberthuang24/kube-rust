/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1VolumeMount : VolumeMount describes a mounting of a Volume within a container.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1VolumeMount {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
    #[serde(rename = "mountPropagation", skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<String>,
    /// This must match the Name of a Volume.
    #[serde(rename = "name")]
    pub name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to \"\" (volume's root).
    #[serde(rename = "subPath", skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to \"\" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    #[serde(rename = "subPathExpr", skip_serializing_if = "Option::is_none")]
    pub sub_path_expr: Option<String>,
}

impl V1VolumeMount {
    /// VolumeMount describes a mounting of a Volume within a container.
    pub fn new(mount_path: String, name: String) -> V1VolumeMount {
        V1VolumeMount {
            mount_path,
            mount_propagation: None,
            name,
            read_only: None,
            sub_path: None,
            sub_path_expr: None,
        }
    }
}


