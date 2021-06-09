/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1NodeSelector : A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1NodeSelector {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<crate::models::V1NodeSelectorTerm>,
}

impl V1NodeSelector {
    /// A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
    pub fn new(node_selector_terms: Vec<crate::models::V1NodeSelectorTerm>) -> V1NodeSelector {
        V1NodeSelector {
            node_selector_terms,
        }
    }
}


