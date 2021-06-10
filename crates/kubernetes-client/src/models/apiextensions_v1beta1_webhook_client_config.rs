/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApiextensionsV1beta1WebhookClientConfig : WebhookClientConfig contains the information to make a TLS connection with the webhook.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiextensionsV1beta1WebhookClientConfig {
    /// caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate. If unspecified, system trust roots on the apiserver are used.
    #[serde(rename = "caBundle", skip_serializing_if = "Option::is_none")]
    pub ca_bundle: Option<String>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<crate::models::ApiextensionsV1beta1ServiceReference>>,
    /// url gives the location of the webhook, in standard URL form (`scheme://host:port/path`). Exactly one of `url` or `service` must be specified.  The `host` should not refer to a service running in the cluster; use the `service` field instead. The host might be resolved via external DNS in some apiservers (e.g., `kube-apiserver` cannot resolve in-cluster DNS as that would be a layering violation). `host` may also be an IP address.  Please note that using `localhost` or `127.0.0.1` as a `host` is risky unless you take great care to run this webhook on all hosts which run an apiserver which might need to make calls to this webhook. Such installs are likely to be non-portable, i.e., not easy to turn up in a new cluster.  The scheme must be \"https\"; the URL must begin with \"https://\".  A path is optional, and if present may be any string permissible in a URL. You may use the path to pass an arbitrary string to the webhook, for example, a cluster identifier.  Attempting to use a user or basic auth e.g. \"user:password@\" is not allowed. Fragments (\"#...\") and query parameters (\"?...\") are not allowed, either.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ApiextensionsV1beta1WebhookClientConfig {
    /// WebhookClientConfig contains the information to make a TLS connection with the webhook.
    pub fn new() -> ApiextensionsV1beta1WebhookClientConfig {
        ApiextensionsV1beta1WebhookClientConfig {
            ca_bundle: None,
            service: None,
            url: None,
        }
    }
}


