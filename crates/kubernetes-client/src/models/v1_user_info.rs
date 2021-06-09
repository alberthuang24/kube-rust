/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1UserInfo : UserInfo holds the information about the user needed to implement the user.Info interface.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1UserInfo {
    /// Any additional information provided by the authenticator.
    #[serde(rename = "extra", skip_serializing_if = "Option::is_none")]
    pub extra: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// The names of groups this user is a part of.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// A unique value that identifies this user across time. If this user is deleted and another user by the same name is added, they will have different UIDs.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// The name that uniquely identifies this user among all active users.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl V1UserInfo {
    /// UserInfo holds the information about the user needed to implement the user.Info interface.
    pub fn new() -> V1UserInfo {
        V1UserInfo {
            extra: None,
            groups: None,
            uid: None,
            username: None,
        }
    }
}


