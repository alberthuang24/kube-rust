/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1NonResourcePolicyRule : NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member of verbs matches the request and (b) at least one member of nonResourceURLs matches the request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1NonResourcePolicyRule {
    /// `nonResourceURLs` is a set of url prefixes that a user should have access to and may not be empty. For example:   - \"/healthz\" is legal   - \"/hea*\" is illegal   - \"/hea\" is legal but matches nothing   - \"/hea/_*\" also matches nothing   - \"/healthz/_*\" matches all per-component health checks. \"*\" matches all non-resource urls. if it is present, it must be the only entry. Required.
    #[serde(rename = "nonResourceURLs")]
    pub non_resource_ur_ls: Vec<String>,
    /// `verbs` is a list of matching verbs and may not be empty. \"*\" matches all verbs. If it is present, it must be the only entry. Required.
    #[serde(rename = "verbs")]
    pub verbs: Vec<String>,
}

impl V1beta1NonResourcePolicyRule {
    /// NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member of verbs matches the request and (b) at least one member of nonResourceURLs matches the request.
    pub fn new(non_resource_ur_ls: Vec<String>, verbs: Vec<String>) -> V1beta1NonResourcePolicyRule {
        V1beta1NonResourcePolicyRule {
            non_resource_ur_ls,
            verbs,
        }
    }
}


