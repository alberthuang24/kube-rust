/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1ContainerStatus : ContainerStatus contains details for the current status of this container.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1ContainerStatus {
    /// Container's ID in the format 'docker://<container_id>'.
    #[serde(rename = "containerID", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images
    #[serde(rename = "image")]
    pub image: String,
    /// ImageID of the container's image.
    #[serde(rename = "imageID")]
    pub image_id: String,
    #[serde(rename = "lastState", skip_serializing_if = "Option::is_none")]
    pub last_state: Option<Box<crate::models::V1ContainerState>>,
    /// This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies whether the container has passed its readiness probe.
    #[serde(rename = "ready")]
    pub ready: bool,
    /// The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC.
    #[serde(rename = "restartCount")]
    pub restart_count: i32,
    /// Specifies whether the container has passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. Is always true when no startupProbe is defined.
    #[serde(rename = "started", skip_serializing_if = "Option::is_none")]
    pub started: Option<bool>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<crate::models::V1ContainerState>>,
}

impl V1ContainerStatus {
    /// ContainerStatus contains details for the current status of this container.
    pub fn new(image: String, image_id: String, name: String, ready: bool, restart_count: i32) -> V1ContainerStatus {
        V1ContainerStatus {
            container_id: None,
            image,
            image_id,
            last_state: None,
            name,
            ready,
            restart_count,
            started: None,
            state: None,
        }
    }
}


