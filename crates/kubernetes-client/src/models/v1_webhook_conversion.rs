/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1WebhookConversion : WebhookConversion describes how to call a conversion webhook



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1WebhookConversion {
    #[serde(rename = "clientConfig", skip_serializing_if = "Option::is_none")]
    pub client_config: Option<crate::models::ApiextensionsV1WebhookClientConfig>,
    /// conversionReviewVersions is an ordered list of preferred `ConversionReview` versions the Webhook expects. The API server will use the first version in the list which it supports. If none of the versions specified in this list are supported by API server, conversion will fail for the custom resource. If a persisted Webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail.
    #[serde(rename = "conversionReviewVersions")]
    pub conversion_review_versions: Vec<String>,
}

impl V1WebhookConversion {
    /// WebhookConversion describes how to call a conversion webhook
    pub fn new(conversion_review_versions: Vec<String>) -> V1WebhookConversion {
        V1WebhookConversion {
            client_config: None,
            conversion_review_versions,
        }
    }
}


