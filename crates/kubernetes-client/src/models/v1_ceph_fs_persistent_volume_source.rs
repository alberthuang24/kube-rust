/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CephFsPersistentVolumeSource : Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CephFsPersistentVolumeSource {
    /// Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "monitors")]
    pub monitors: Vec<String>,
    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "secretFile", skip_serializing_if = "Option::is_none")]
    pub secret_file: Option<String>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<crate::models::V1SecretReference>,
    /// Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl V1CephFsPersistentVolumeSource {
    /// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
    pub fn new(monitors: Vec<String>) -> V1CephFsPersistentVolumeSource {
        V1CephFsPersistentVolumeSource {
            monitors,
            path: None,
            read_only: None,
            secret_file: None,
            secret_ref: None,
            user: None,
        }
    }
}


