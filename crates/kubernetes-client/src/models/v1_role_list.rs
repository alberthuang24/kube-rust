/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1RoleList : RoleList is a collection of Roles



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1RoleList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Items is a list of Roles
    #[serde(rename = "items")]
    pub items: Vec<crate::models::V1Role>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::V1ListMeta>,
}

impl V1RoleList {
    /// RoleList is a collection of Roles
    pub fn new(items: Vec<crate::models::V1Role>) -> V1RoleList {
        V1RoleList {
            api_version: None,
            items,
            kind: None,
            metadata: None,
        }
    }
}


