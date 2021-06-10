/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1DaemonSetList : DaemonSetList is a collection of daemon sets.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1DaemonSetList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// A list of daemon sets.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::V1DaemonSet>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ListMeta>>,
}

impl V1DaemonSetList {
    /// DaemonSetList is a collection of daemon sets.
    pub fn new(items: Vec<crate::models::V1DaemonSet>) -> V1DaemonSetList {
        V1DaemonSetList {
            api_version: None,
            items,
            kind: None,
            metadata: None,
        }
    }
}


