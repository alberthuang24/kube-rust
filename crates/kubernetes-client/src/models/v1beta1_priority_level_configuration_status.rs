/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1PriorityLevelConfigurationStatus : PriorityLevelConfigurationStatus represents the current state of a \"request-priority\".



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1PriorityLevelConfigurationStatus {
    /// `conditions` is the current state of \"request-priority\".
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::V1beta1PriorityLevelConfigurationCondition>>,
}

impl V1beta1PriorityLevelConfigurationStatus {
    /// PriorityLevelConfigurationStatus represents the current state of a \"request-priority\".
    pub fn new() -> V1beta1PriorityLevelConfigurationStatus {
        V1beta1PriorityLevelConfigurationStatus {
            conditions: None,
        }
    }
}

