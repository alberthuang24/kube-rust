/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1AggregationRule : AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1AggregationRule {
    /// ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
    #[serde(rename = "clusterRoleSelectors", skip_serializing_if = "Option::is_none")]
    pub cluster_role_selectors: Option<Vec<crate::models::V1LabelSelector>>,
}

impl V1beta1AggregationRule {
    /// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
    pub fn new() -> V1beta1AggregationRule {
        V1beta1AggregationRule {
            cluster_role_selectors: None,
        }
    }
}


