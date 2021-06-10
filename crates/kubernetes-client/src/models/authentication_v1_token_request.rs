/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthenticationV1TokenRequest : TokenRequest requests a token for a given service account.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationV1TokenRequest {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ObjectMeta>>,
    #[serde(rename = "spec")]
    pub spec: Box<crate::models::V1TokenRequestSpec>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::V1TokenRequestStatus>>,
}

impl AuthenticationV1TokenRequest {
    /// TokenRequest requests a token for a given service account.
    pub fn new(spec: crate::models::V1TokenRequestSpec) -> AuthenticationV1TokenRequest {
        AuthenticationV1TokenRequest {
            api_version: None,
            kind: None,
            metadata: None,
            spec: Box::new(spec),
            status: None,
        }
    }
}


