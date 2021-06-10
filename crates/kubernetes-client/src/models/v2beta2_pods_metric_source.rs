/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta2PodsMetricSource : PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta2PodsMetricSource {
    #[serde(rename = "metric")]
    pub metric: Box<crate::models::V2beta2MetricIdentifier>,
    #[serde(rename = "target")]
    pub target: Box<crate::models::V2beta2MetricTarget>,
}

impl V2beta2PodsMetricSource {
    /// PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
    pub fn new(metric: crate::models::V2beta2MetricIdentifier, target: crate::models::V2beta2MetricTarget) -> V2beta2PodsMetricSource {
        V2beta2PodsMetricSource {
            metric: Box::new(metric),
            target: Box::new(target),
        }
    }
}


