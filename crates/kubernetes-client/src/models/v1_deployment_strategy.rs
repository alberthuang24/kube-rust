/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1DeploymentStrategy : DeploymentStrategy describes how to replace existing pods with new ones.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1DeploymentStrategy {
    #[serde(rename = "rollingUpdate", skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<Box<crate::models::V1RollingUpdateDeployment>>,
    /// Type of deployment. Can be \"Recreate\" or \"RollingUpdate\". Default is RollingUpdate.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl V1DeploymentStrategy {
    /// DeploymentStrategy describes how to replace existing pods with new ones.
    pub fn new() -> V1DeploymentStrategy {
        V1DeploymentStrategy {
            rolling_update: None,
            _type: None,
        }
    }
}


