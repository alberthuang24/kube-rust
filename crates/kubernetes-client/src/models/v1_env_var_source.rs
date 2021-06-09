/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1EnvVarSource : EnvVarSource represents a source for the value of an EnvVar.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1EnvVarSource {
    #[serde(rename = "configMapKeyRef", skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<crate::models::V1ConfigMapKeySelector>,
    #[serde(rename = "fieldRef", skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<crate::models::V1ObjectFieldSelector>,
    #[serde(rename = "resourceFieldRef", skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<crate::models::V1ResourceFieldSelector>,
    #[serde(rename = "secretKeyRef", skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<crate::models::V1SecretKeySelector>,
}

impl V1EnvVarSource {
    /// EnvVarSource represents a source for the value of an EnvVar.
    pub fn new() -> V1EnvVarSource {
        V1EnvVarSource {
            config_map_key_ref: None,
            field_ref: None,
            resource_field_ref: None,
            secret_key_ref: None,
        }
    }
}


