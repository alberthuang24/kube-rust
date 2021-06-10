/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1alpha1RuntimeClassSpec : RuntimeClassSpec is a specification of a RuntimeClass. It contains parameters that are required to describe the RuntimeClass to the Container Runtime Interface (CRI) implementation, as well as any other components that need to understand how the pod will be run. The RuntimeClassSpec is immutable.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1alpha1RuntimeClassSpec {
    #[serde(rename = "overhead", skip_serializing_if = "Option::is_none")]
    pub overhead: Option<Box<crate::models::V1alpha1Overhead>>,
    /// RuntimeHandler specifies the underlying runtime and configuration that the CRI implementation will use to handle pods of this class. The possible values are specific to the node & CRI configuration.  It is assumed that all handlers are available on every node, and handlers of the same name are equivalent on every node. For example, a handler called \"runc\" might specify that the runc OCI runtime (using native Linux containers) will be used to run the containers in a pod. The RuntimeHandler must be lowercase, conform to the DNS Label (RFC 1123) requirements, and is immutable.
    #[serde(rename = "runtimeHandler")]
    pub runtime_handler: String,
    #[serde(rename = "scheduling", skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Box<crate::models::V1alpha1Scheduling>>,
}

impl V1alpha1RuntimeClassSpec {
    /// RuntimeClassSpec is a specification of a RuntimeClass. It contains parameters that are required to describe the RuntimeClass to the Container Runtime Interface (CRI) implementation, as well as any other components that need to understand how the pod will be run. The RuntimeClassSpec is immutable.
    pub fn new(runtime_handler: String) -> V1alpha1RuntimeClassSpec {
        V1alpha1RuntimeClassSpec {
            overhead: None,
            runtime_handler,
            scheduling: None,
        }
    }
}


