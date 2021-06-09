/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ResourceQuota : ResourceQuota sets aggregate quota restrictions enforced per namespace



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ResourceQuota {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::V1ObjectMeta>,
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<crate::models::V1ResourceQuotaSpec>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::V1ResourceQuotaStatus>,
}

impl V1ResourceQuota {
    /// ResourceQuota sets aggregate quota restrictions enforced per namespace
    pub fn new() -> V1ResourceQuota {
        V1ResourceQuota {
            api_version: None,
            kind: None,
            metadata: None,
            spec: None,
            status: None,
        }
    }
}


