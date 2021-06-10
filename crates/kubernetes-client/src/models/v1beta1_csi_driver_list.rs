/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1CsiDriverList : CSIDriverList is a collection of CSIDriver objects.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1CsiDriverList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// items is the list of CSIDriver
    #[serde(rename = "items")]
    pub items: Vec<crate::models::V1beta1CsiDriver>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ListMeta>>,
}

impl V1beta1CsiDriverList {
    /// CSIDriverList is a collection of CSIDriver objects.
    pub fn new(items: Vec<crate::models::V1beta1CsiDriver>) -> V1beta1CsiDriverList {
        V1beta1CsiDriverList {
            api_version: None,
            items,
            kind: None,
            metadata: None,
        }
    }
}


