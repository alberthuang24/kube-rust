/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ScaleSpec : ScaleSpec describes the attributes of a scale subresource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ScaleSpec {
    /// desired number of instances for the scaled object.
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

impl V1ScaleSpec {
    /// ScaleSpec describes the attributes of a scale subresource.
    pub fn new() -> V1ScaleSpec {
        V1ScaleSpec {
            replicas: None,
        }
    }
}

