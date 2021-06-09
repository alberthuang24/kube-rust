/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1AwsElasticBlockStoreVolumeSource : Represents a Persistent Disk resource in AWS.  An AWS EBS disk must exist before mounting to a container. The disk must also be in the same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS EBS volumes support ownership management and SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1AwsElasticBlockStoreVolumeSource {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as \"1\". Similarly, the volume partition for /dev/sda is \"0\" (or you can leave the property empty).
    #[serde(rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    /// Specify \"true\" to force and set the ReadOnly property in VolumeMounts to \"true\". If omitted, the default is \"false\". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(rename = "volumeID")]
    pub volume_id: String,
}

impl V1AwsElasticBlockStoreVolumeSource {
    /// Represents a Persistent Disk resource in AWS.  An AWS EBS disk must exist before mounting to a container. The disk must also be in the same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS EBS volumes support ownership management and SELinux relabeling.
    pub fn new(volume_id: String) -> V1AwsElasticBlockStoreVolumeSource {
        V1AwsElasticBlockStoreVolumeSource {
            fs_type: None,
            partition: None,
            read_only: None,
            volume_id,
        }
    }
}


