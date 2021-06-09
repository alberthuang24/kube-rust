/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1SelfSubjectAccessReviewSpec : SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1SelfSubjectAccessReviewSpec {
    #[serde(rename = "nonResourceAttributes", skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<crate::models::V1NonResourceAttributes>,
    #[serde(rename = "resourceAttributes", skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<crate::models::V1ResourceAttributes>,
}

impl V1SelfSubjectAccessReviewSpec {
    /// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
    pub fn new() -> V1SelfSubjectAccessReviewSpec {
        V1SelfSubjectAccessReviewSpec {
            non_resource_attributes: None,
            resource_attributes: None,
        }
    }
}


