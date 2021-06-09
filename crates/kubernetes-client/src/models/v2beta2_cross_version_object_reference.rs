/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V2beta2CrossVersionObjectReference : CrossVersionObjectReference contains enough information to let you identify the referred resource.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V2beta2CrossVersionObjectReference {
    /// API version of the referent
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds\"
    #[serde(rename = "kind")]
    pub kind: String,
    /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
    #[serde(rename = "name")]
    pub name: String,
}

impl V2beta2CrossVersionObjectReference {
    /// CrossVersionObjectReference contains enough information to let you identify the referred resource.
    pub fn new(kind: String, name: String) -> V2beta2CrossVersionObjectReference {
        V2beta2CrossVersionObjectReference {
            api_version: None,
            kind,
            name,
        }
    }
}


