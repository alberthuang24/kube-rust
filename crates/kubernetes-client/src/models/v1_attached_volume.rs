/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1AttachedVolume : AttachedVolume describes a volume attached to a node



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1AttachedVolume {
    /// DevicePath represents the device path where the volume should be available
    #[serde(rename = "devicePath")]
    pub device_path: String,
    /// Name of the attached volume
    #[serde(rename = "name")]
    pub name: String,
}

impl V1AttachedVolume {
    /// AttachedVolume describes a volume attached to a node
    pub fn new(device_path: String, name: String) -> V1AttachedVolume {
        V1AttachedVolume {
            device_path,
            name,
        }
    }
}


