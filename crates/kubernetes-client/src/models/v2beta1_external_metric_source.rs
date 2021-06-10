/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta1ExternalMetricSource : ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster). Exactly one \"target\" type should be set.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta1ExternalMetricSource {
    /// metricName is the name of the metric in question.
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[serde(rename = "metricSelector", skip_serializing_if = "Option::is_none")]
    pub metric_selector: Option<Box<crate::models::V1LabelSelector>>,
    /// targetAverageValue is the target per-pod value of global metric (as a quantity). Mutually exclusive with TargetValue.
    #[serde(rename = "targetAverageValue", skip_serializing_if = "Option::is_none")]
    pub target_average_value: Option<String>,
    /// targetValue is the target value of the metric (as a quantity). Mutually exclusive with TargetAverageValue.
    #[serde(rename = "targetValue", skip_serializing_if = "Option::is_none")]
    pub target_value: Option<String>,
}

impl V2beta1ExternalMetricSource {
    /// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster). Exactly one \"target\" type should be set.
    pub fn new(metric_name: String) -> V2beta1ExternalMetricSource {
        V2beta1ExternalMetricSource {
            metric_name,
            metric_selector: None,
            target_average_value: None,
            target_value: None,
        }
    }
}


