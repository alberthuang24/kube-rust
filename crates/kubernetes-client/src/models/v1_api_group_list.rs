/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ApiGroupList : APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ApiGroupList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// groups is a list of APIGroup.
    #[serde(rename = "groups")]
    pub groups: Vec<crate::models::V1ApiGroup>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl V1ApiGroupList {
    /// APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.
    pub fn new(groups: Vec<crate::models::V1ApiGroup>) -> V1ApiGroupList {
        V1ApiGroupList {
            api_version: None,
            groups,
            kind: None,
        }
    }
}


