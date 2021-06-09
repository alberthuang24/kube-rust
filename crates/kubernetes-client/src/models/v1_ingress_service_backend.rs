/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IngressServiceBackend : IngressServiceBackend references a Kubernetes Service as a Backend.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1IngressServiceBackend {
    /// Name is the referenced service. The service must exist in the same namespace as the Ingress object.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<crate::models::V1ServiceBackendPort>,
}

impl V1IngressServiceBackend {
    /// IngressServiceBackend references a Kubernetes Service as a Backend.
    pub fn new(name: String) -> V1IngressServiceBackend {
        V1IngressServiceBackend {
            name,
            port: None,
        }
    }
}


