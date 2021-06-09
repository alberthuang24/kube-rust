/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta2ContainerResourceMetricStatus : ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta2ContainerResourceMetricStatus {
    /// Container is the name of the container in the pods of the scaling target
    #[serde(rename = "container")]
    pub container: String,
    #[serde(rename = "current")]
    pub current: crate::models::V2beta2MetricValueStatus,
    /// Name is the name of the resource in question.
    #[serde(rename = "name")]
    pub name: String,
}

impl V2beta2ContainerResourceMetricStatus {
    /// ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \"pods\" source.
    pub fn new(container: String, current: crate::models::V2beta2MetricValueStatus, name: String) -> V2beta2ContainerResourceMetricStatus {
        V2beta2ContainerResourceMetricStatus {
            container,
            current,
            name,
        }
    }
}


