/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CustomResourceConversion : CustomResourceConversion describes how to convert different versions of a CR.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CustomResourceConversion {
    /// strategy specifies how custom resources are converted between versions. Allowed values are: - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource. - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information   is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhook to be set.
    #[serde(rename = "strategy")]
    pub strategy: String,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Box<crate::models::V1WebhookConversion>>,
}

impl V1CustomResourceConversion {
    /// CustomResourceConversion describes how to convert different versions of a CR.
    pub fn new(strategy: String) -> V1CustomResourceConversion {
        V1CustomResourceConversion {
            strategy,
            webhook: None,
        }
    }
}


