/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta2ResourceMetricSource : ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  The values will be averaged together before being compared to the target.  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.  Only one \"target\" type should be set.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta2ResourceMetricSource {
    /// name is the name of the resource in question.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "target")]
    pub target: Box<crate::models::V2beta2MetricTarget>,
}

impl V2beta2ResourceMetricSource {
    /// ResourceMetricSource indicates how to scale on a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  The values will be averaged together before being compared to the target.  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.  Only one \"target\" type should be set.
    pub fn new(name: String, target: crate::models::V2beta2MetricTarget) -> V2beta2ResourceMetricSource {
        V2beta2ResourceMetricSource {
            name,
            target: Box::new(target),
        }
    }
}


