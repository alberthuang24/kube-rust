/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExtensionsV1beta1IngressSpec : IngressSpec describes the Ingress the user wishes to exist.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionsV1beta1IngressSpec {
    #[serde(rename = "backend", skip_serializing_if = "Option::is_none")]
    pub backend: Option<crate::models::ExtensionsV1beta1IngressBackend>,
    /// IngressClassName is the name of the IngressClass cluster resource. The associated IngressClass defines which controller will implement the resource. This replaces the deprecated `kubernetes.io/ingress.class` annotation. For backwards compatibility, when that annotation is set, it must be given precedence over this field. The controller may emit a warning if the field and annotation have different values. Implementations of this API should ignore Ingresses without a class specified. An IngressClass resource may be marked as default, which can be used to set a default value for this field. For more information, refer to the IngressClass documentation.
    #[serde(rename = "ingressClassName", skip_serializing_if = "Option::is_none")]
    pub ingress_class_name: Option<String>,
    /// A list of host rules used to configure the Ingress. If unspecified, or no rule matches, all traffic is sent to the default backend.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::ExtensionsV1beta1IngressRule>>,
    /// TLS configuration. Currently the Ingress only supports a single TLS port, 443. If multiple members of this list specify different hosts, they will be multiplexed on the same port according to the hostname specified through the SNI TLS extension, if the ingress controller fulfilling the ingress supports SNI.
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<crate::models::ExtensionsV1beta1IngressTls>>,
}

impl ExtensionsV1beta1IngressSpec {
    /// IngressSpec describes the Ingress the user wishes to exist.
    pub fn new() -> ExtensionsV1beta1IngressSpec {
        ExtensionsV1beta1IngressSpec {
            backend: None,
            ingress_class_name: None,
            rules: None,
            tls: None,
        }
    }
}


