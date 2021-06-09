/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1CsiStorageCapacityList : CSIStorageCapacityList is a collection of CSIStorageCapacity objects.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1alpha1CsiStorageCapacityList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Items is the list of CSIStorageCapacity objects.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::V1alpha1CsiStorageCapacity>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::V1ListMeta>,
}

impl V1alpha1CsiStorageCapacityList {
    /// CSIStorageCapacityList is a collection of CSIStorageCapacity objects.
    pub fn new(items: Vec<crate::models::V1alpha1CsiStorageCapacity>) -> V1alpha1CsiStorageCapacityList {
        V1alpha1CsiStorageCapacityList {
            api_version: None,
            items,
            kind: None,
            metadata: None,
        }
    }
}


