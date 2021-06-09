/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta2HorizontalPodAutoscalerSpec : HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta2HorizontalPodAutoscalerSpec {
    #[serde(rename = "behavior", skip_serializing_if = "Option::is_none")]
    pub behavior: Option<crate::models::V2beta2HorizontalPodAutoscalerBehavior>,
    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
    #[serde(rename = "maxReplicas")]
    pub max_replicas: i32,
    /// metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond. If not set, the default metric will be set to 80% average CPU utilization.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<crate::models::V2beta2MetricSpec>>,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
    #[serde(rename = "minReplicas", skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    #[serde(rename = "scaleTargetRef")]
    pub scale_target_ref: crate::models::V2beta2CrossVersionObjectReference,
}

impl V2beta2HorizontalPodAutoscalerSpec {
    /// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
    pub fn new(max_replicas: i32, scale_target_ref: crate::models::V2beta2CrossVersionObjectReference) -> V2beta2HorizontalPodAutoscalerSpec {
        V2beta2HorizontalPodAutoscalerSpec {
            behavior: None,
            max_replicas,
            metrics: None,
            min_replicas: None,
            scale_target_ref,
        }
    }
}


