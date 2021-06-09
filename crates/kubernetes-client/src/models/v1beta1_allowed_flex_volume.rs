/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1AllowedFlexVolume : AllowedFlexVolume represents a single Flexvolume that is allowed to be used.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1AllowedFlexVolume {
    /// driver is the name of the Flexvolume driver.
    #[serde(rename = "driver")]
    pub driver: String,
}

impl V1beta1AllowedFlexVolume {
    /// AllowedFlexVolume represents a single Flexvolume that is allowed to be used.
    pub fn new(driver: String) -> V1beta1AllowedFlexVolume {
        V1beta1AllowedFlexVolume {
            driver,
        }
    }
}

