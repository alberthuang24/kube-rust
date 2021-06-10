/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NetworkPolicyList : NetworkPolicyList is a list of NetworkPolicy objects.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1NetworkPolicyList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Items is a list of schema objects.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::V1NetworkPolicy>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ListMeta>>,
}

impl V1NetworkPolicyList {
    /// NetworkPolicyList is a list of NetworkPolicy objects.
    pub fn new(items: Vec<crate::models::V1NetworkPolicy>) -> V1NetworkPolicyList {
        V1NetworkPolicyList {
            api_version: None,
            items,
            kind: None,
            metadata: None,
        }
    }
}


