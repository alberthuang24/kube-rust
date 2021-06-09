/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1RoleRef : RoleRef contains information that points to the role being used



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1RoleRef {
    /// APIGroup is the group for the resource being referenced
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    /// Kind is the type of resource being referenced
    #[serde(rename = "kind")]
    pub kind: String,
    /// Name is the name of resource being referenced
    #[serde(rename = "name")]
    pub name: String,
}

impl V1beta1RoleRef {
    /// RoleRef contains information that points to the role being used
    pub fn new(api_group: String, kind: String, name: String) -> V1beta1RoleRef {
        V1beta1RoleRef {
            api_group,
            kind,
            name,
        }
    }
}


