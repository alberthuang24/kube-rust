/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1SecretReference : SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1SecretReference {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl V1SecretReference {
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
    pub fn new() -> V1SecretReference {
        V1SecretReference {
            name: None,
            namespace: None,
        }
    }
}


