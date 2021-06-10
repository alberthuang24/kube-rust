/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta2ExternalMetricSource : ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta2ExternalMetricSource {
    #[serde(rename = "metric")]
    pub metric: Box<crate::models::V2beta2MetricIdentifier>,
    #[serde(rename = "target")]
    pub target: Box<crate::models::V2beta2MetricTarget>,
}

impl V2beta2ExternalMetricSource {
    /// ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
    pub fn new(metric: crate::models::V2beta2MetricIdentifier, target: crate::models::V2beta2MetricTarget) -> V2beta2ExternalMetricSource {
        V2beta2ExternalMetricSource {
            metric: Box::new(metric),
            target: Box::new(target),
        }
    }
}


