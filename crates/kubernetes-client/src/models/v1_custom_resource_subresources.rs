/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CustomResourceSubresources : CustomResourceSubresources defines the status and scale subresources for CustomResources.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CustomResourceSubresources {
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<Box<crate::models::V1CustomResourceSubresourceScale>>,
    /// status indicates the custom resource should serve a `/status` subresource. When enabled: 1. requests to the custom resource primary endpoint ignore changes to the `status` stanza of the object. 2. requests to the custom resource `/status` subresource ignore changes to anything other than the `status` stanza of the object.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
}

impl V1CustomResourceSubresources {
    /// CustomResourceSubresources defines the status and scale subresources for CustomResources.
    pub fn new() -> V1CustomResourceSubresources {
        V1CustomResourceSubresources {
            scale: None,
            status: None,
        }
    }
}


