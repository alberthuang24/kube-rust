/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta2MetricSpec : MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta2MetricSpec {
    #[serde(rename = "containerResource", skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<Box<crate::models::V2beta2ContainerResourceMetricSource>>,
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<Box<crate::models::V2beta2ExternalMetricSource>>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<crate::models::V2beta2ObjectMetricSource>>,
    #[serde(rename = "pods", skip_serializing_if = "Option::is_none")]
    pub pods: Option<Box<crate::models::V2beta2PodsMetricSource>>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<crate::models::V2beta2ResourceMetricSource>>,
    /// type is the type of metric source.  It should be one of \"ContainerResource\", \"External\", \"Object\", \"Pods\" or \"Resource\", each mapping to a matching field in the object. Note: \"ContainerResource\" type is available on when the feature-gate HPAContainerMetrics is enabled
    #[serde(rename = "type")]
    pub _type: String,
}

impl V2beta2MetricSpec {
    /// MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).
    pub fn new(_type: String) -> V2beta2MetricSpec {
        V2beta2MetricSpec {
            container_resource: None,
            external: None,
            object: None,
            pods: None,
            resource: None,
            _type,
        }
    }
}


