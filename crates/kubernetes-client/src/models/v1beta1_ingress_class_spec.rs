/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1IngressClassSpec : IngressClassSpec provides information about the class of an Ingress.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1IngressClassSpec {
    /// Controller refers to the name of the controller that should handle this class. This allows for different \"flavors\" that are controlled by the same controller. For example, you may have different Parameters for the same implementing controller. This should be specified as a domain-prefixed path no more than 250 characters in length, e.g. \"acme.io/ingress-controller\". This field is immutable.
    #[serde(rename = "controller", skip_serializing_if = "Option::is_none")]
    pub controller: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<crate::models::V1beta1IngressClassParametersReference>>,
}

impl V1beta1IngressClassSpec {
    /// IngressClassSpec provides information about the class of an Ingress.
    pub fn new() -> V1beta1IngressClassSpec {
        V1beta1IngressClassSpec {
            controller: None,
            parameters: None,
        }
    }
}


