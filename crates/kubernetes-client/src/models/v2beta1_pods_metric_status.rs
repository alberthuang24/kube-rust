/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta1PodsMetricStatus : PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta1PodsMetricStatus {
    /// currentAverageValue is the current value of the average of the metric across all relevant pods (as a quantity)
    #[serde(rename = "currentAverageValue")]
    pub current_average_value: String,
    /// metricName is the name of the metric in question
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<crate::models::V1LabelSelector>,
}

impl V2beta1PodsMetricStatus {
    /// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
    pub fn new(current_average_value: String, metric_name: String) -> V2beta1PodsMetricStatus {
        V2beta1PodsMetricStatus {
            current_average_value,
            metric_name,
            selector: None,
        }
    }
}


