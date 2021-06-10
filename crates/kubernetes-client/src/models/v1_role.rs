/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1Role : Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1Role {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ObjectMeta>>,
    /// Rules holds all the PolicyRules for this Role
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::V1PolicyRule>>,
}

impl V1Role {
    /// Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.
    pub fn new() -> V1Role {
        V1Role {
            api_version: None,
            kind: None,
            metadata: None,
            rules: None,
        }
    }
}


