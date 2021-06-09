/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IngressClassParametersReference : IngressClassParametersReference identifies an API object. This can be used to specify a cluster or namespace-scoped resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1IngressClassParametersReference {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(rename = "apiGroup", skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced.
    #[serde(rename = "kind")]
    pub kind: String,
    /// Name is the name of resource being referenced.
    #[serde(rename = "name")]
    pub name: String,
    /// Namespace is the namespace of the resource being referenced. This field is required when scope is set to \"Namespace\" and must be unset when scope is set to \"Cluster\".
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Scope represents if this refers to a cluster or namespace scoped resource. This may be set to \"Cluster\" (default) or \"Namespace\". Field can be enabled with IngressClassNamespacedParams feature gate.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl V1IngressClassParametersReference {
    /// IngressClassParametersReference identifies an API object. This can be used to specify a cluster or namespace-scoped resource.
    pub fn new(kind: String, name: String) -> V1IngressClassParametersReference {
        V1IngressClassParametersReference {
            api_group: None,
            kind,
            name,
            namespace: None,
            scope: None,
        }
    }
}


