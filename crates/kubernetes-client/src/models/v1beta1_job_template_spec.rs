/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1JobTemplateSpec : JobTemplateSpec describes the data a Job should have when created from a template



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1JobTemplateSpec {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::V1ObjectMeta>>,
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::V1JobSpec>>,
}

impl V1beta1JobTemplateSpec {
    /// JobTemplateSpec describes the data a Job should have when created from a template
    pub fn new() -> V1beta1JobTemplateSpec {
        V1beta1JobTemplateSpec {
            metadata: None,
            spec: None,
        }
    }
}


