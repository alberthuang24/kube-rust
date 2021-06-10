/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ReplicaSetSpec : ReplicaSetSpec is the specification of a ReplicaSet.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ReplicaSetSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(rename = "minReadySeconds", skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(rename = "selector")]
    pub selector: Box<crate::models::V1LabelSelector>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::V1PodTemplateSpec>>,
}

impl V1ReplicaSetSpec {
    /// ReplicaSetSpec is the specification of a ReplicaSet.
    pub fn new(selector: crate::models::V1LabelSelector) -> V1ReplicaSetSpec {
        V1ReplicaSetSpec {
            min_ready_seconds: None,
            replicas: None,
            selector: Box::new(selector),
            template: None,
        }
    }
}


