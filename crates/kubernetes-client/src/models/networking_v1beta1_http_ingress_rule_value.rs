/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NetworkingV1beta1HttpIngressRuleValue : HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://<host>/<path>?<searchpart> -> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkingV1beta1HttpIngressRuleValue {
    /// A collection of paths that map requests to backends.
    #[serde(rename = "paths")]
    pub paths: Vec<crate::models::NetworkingV1beta1HttpIngressPath>,
}

impl NetworkingV1beta1HttpIngressRuleValue {
    /// HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://<host>/<path>?<searchpart> -> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.
    pub fn new(paths: Vec<crate::models::NetworkingV1beta1HttpIngressPath>) -> NetworkingV1beta1HttpIngressRuleValue {
        NetworkingV1beta1HttpIngressRuleValue {
            paths,
        }
    }
}


