/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1WeightedPodAffinityTerm : The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1WeightedPodAffinityTerm {
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: crate::models::V1PodAffinityTerm,
    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    #[serde(rename = "weight")]
    pub weight: i32,
}

impl V1WeightedPodAffinityTerm {
    /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
    pub fn new(pod_affinity_term: crate::models::V1PodAffinityTerm, weight: i32) -> V1WeightedPodAffinityTerm {
        V1WeightedPodAffinityTerm {
            pod_affinity_term,
            weight,
        }
    }
}


