/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1SeLinuxStrategyOptions : SELinuxStrategyOptions defines the strategy type and any options used to create the strategy.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1SeLinuxStrategyOptions {
    /// rule is the strategy that will dictate the allowable labels that may be set.
    #[serde(rename = "rule")]
    pub rule: String,
    #[serde(rename = "seLinuxOptions", skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<Box<crate::models::V1SeLinuxOptions>>,
}

impl V1beta1SeLinuxStrategyOptions {
    /// SELinuxStrategyOptions defines the strategy type and any options used to create the strategy.
    pub fn new(rule: String) -> V1beta1SeLinuxStrategyOptions {
        V1beta1SeLinuxStrategyOptions {
            rule,
            se_linux_options: None,
        }
    }
}


