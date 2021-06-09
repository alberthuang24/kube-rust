/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1VolumeDevice : volumeDevice describes a mapping of a raw block device within a container.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1VolumeDevice {
    /// devicePath is the path inside of the container that the device will be mapped to.
    #[serde(rename = "devicePath")]
    pub device_path: String,
    /// name must match the name of a persistentVolumeClaim in the pod
    #[serde(rename = "name")]
    pub name: String,
}

impl V1VolumeDevice {
    /// volumeDevice describes a mapping of a raw block device within a container.
    pub fn new(device_path: String, name: String) -> V1VolumeDevice {
        V1VolumeDevice {
            device_path,
            name,
        }
    }
}


