/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CertificateSigningRequestCondition : CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CertificateSigningRequestCondition {
    /// lastTransitionTime is the time the condition last transitioned from one status to another. If unset, when a new condition type is added or an existing condition's status is changed, the server defaults this to the current time.
    #[serde(rename = "lastTransitionTime", skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// lastUpdateTime is the time of the last update to this condition
    #[serde(rename = "lastUpdateTime", skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    /// message contains a human readable message with details about the request state
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// reason indicates a brief reason for the request state
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// status of the condition, one of True, False, Unknown. Approved, Denied, and Failed conditions may not be \"False\" or \"Unknown\".
    #[serde(rename = "status")]
    pub status: String,
    /// type of the condition. Known conditions are \"Approved\", \"Denied\", and \"Failed\".  An \"Approved\" condition is added via the /approval subresource, indicating the request was approved and should be issued by the signer.  A \"Denied\" condition is added via the /approval subresource, indicating the request was denied and should not be issued by the signer.  A \"Failed\" condition is added via the /status subresource, indicating the signer failed to issue the certificate.  Approved and Denied conditions are mutually exclusive. Approved, Denied, and Failed conditions cannot be removed once added.  Only one condition of a given type is allowed.
    #[serde(rename = "type")]
    pub _type: String,
}

impl V1CertificateSigningRequestCondition {
    /// CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object
    pub fn new(status: String, _type: String) -> V1CertificateSigningRequestCondition {
        V1CertificateSigningRequestCondition {
            last_transition_time: None,
            last_update_time: None,
            message: None,
            reason: None,
            status,
            _type,
        }
    }
}

