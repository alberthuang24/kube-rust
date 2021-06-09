/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ApiServiceStatus : APIServiceStatus contains derived information about an API server



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ApiServiceStatus {
    /// Current service state of apiService.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::V1ApiServiceCondition>>,
}

impl V1ApiServiceStatus {
    /// APIServiceStatus contains derived information about an API server
    pub fn new() -> V1ApiServiceStatus {
        V1ApiServiceStatus {
            conditions: None,
        }
    }
}


