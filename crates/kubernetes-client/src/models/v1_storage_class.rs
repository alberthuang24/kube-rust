/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1StorageClass : StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.  StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1StorageClass {
    /// AllowVolumeExpansion shows whether the storage class allow volume expand
    #[serde(rename = "allowVolumeExpansion", skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<bool>,
    /// Restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.
    #[serde(rename = "allowedTopologies", skip_serializing_if = "Option::is_none")]
    pub allowed_topologies: Option<Vec<crate::models::V1TopologySelectorTerm>>,
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::V1ObjectMeta>,
    /// Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. [\"ro\", \"soft\"]. Not validated - mount of the PVs will simply fail if one is invalid.
    #[serde(rename = "mountOptions", skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// Provisioner indicates the type of the provisioner.
    #[serde(rename = "provisioner")]
    pub provisioner: String,
    /// Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete.
    #[serde(rename = "reclaimPolicy", skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<String>,
    /// VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.
    #[serde(rename = "volumeBindingMode", skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: Option<String>,
}

impl V1StorageClass {
    /// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.  StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
    pub fn new(provisioner: String) -> V1StorageClass {
        V1StorageClass {
            allow_volume_expansion: None,
            allowed_topologies: None,
            api_version: None,
            kind: None,
            metadata: None,
            mount_options: None,
            parameters: None,
            provisioner,
            reclaim_policy: None,
            volume_binding_mode: None,
        }
    }
}

