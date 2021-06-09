/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1ServiceAccountSubject : ServiceAccountSubject holds detailed information for service-account-kind subject.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1ServiceAccountSubject {
    /// `name` is the name of matching ServiceAccount objects, or \"*\" to match regardless of name. Required.
    #[serde(rename = "name")]
    pub name: String,
    /// `namespace` is the namespace of matching ServiceAccount objects. Required.
    #[serde(rename = "namespace")]
    pub namespace: String,
}

impl V1beta1ServiceAccountSubject {
    /// ServiceAccountSubject holds detailed information for service-account-kind subject.
    pub fn new(name: String, namespace: String) -> V1beta1ServiceAccountSubject {
        V1beta1ServiceAccountSubject {
            name,
            namespace,
        }
    }
}


