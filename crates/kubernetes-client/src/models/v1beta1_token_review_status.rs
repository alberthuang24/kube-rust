/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1TokenReviewStatus : TokenReviewStatus is the result of the token authentication request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1TokenReviewStatus {
    /// Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is \"true\", the token is valid against the audience of the Kubernetes API server.
    #[serde(rename = "audiences", skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,
    /// Authenticated indicates that the token was associated with a known user.
    #[serde(rename = "authenticated", skip_serializing_if = "Option::is_none")]
    pub authenticated: Option<bool>,
    /// Error indicates that the token couldn't be checked
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::V1beta1UserInfo>,
}

impl V1beta1TokenReviewStatus {
    /// TokenReviewStatus is the result of the token authentication request.
    pub fn new() -> V1beta1TokenReviewStatus {
        V1beta1TokenReviewStatus {
            audiences: None,
            authenticated: None,
            error: None,
            user: None,
        }
    }
}


