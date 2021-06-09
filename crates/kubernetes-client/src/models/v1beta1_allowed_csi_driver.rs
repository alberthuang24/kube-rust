/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1AllowedCsiDriver : AllowedCSIDriver represents a single inline CSI Driver that is allowed to be used.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1AllowedCsiDriver {
    /// Name is the registered name of the CSI driver
    #[serde(rename = "name")]
    pub name: String,
}

impl V1beta1AllowedCsiDriver {
    /// AllowedCSIDriver represents a single inline CSI Driver that is allowed to be used.
    pub fn new(name: String) -> V1beta1AllowedCsiDriver {
        V1beta1AllowedCsiDriver {
            name,
        }
    }
}


