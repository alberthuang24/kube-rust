/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1IscsiVolumeSource : Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1IscsiVolumeSource {
    /// whether support iSCSI Discovery CHAP authentication
    #[serde(rename = "chapAuthDiscovery", skip_serializing_if = "Option::is_none")]
    pub chap_auth_discovery: Option<bool>,
    /// whether support iSCSI Session CHAP authentication
    #[serde(rename = "chapAuthSession", skip_serializing_if = "Option::is_none")]
    pub chap_auth_session: Option<bool>,
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
    #[serde(rename = "initiatorName", skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    /// Target iSCSI Qualified Name.
    #[serde(rename = "iqn")]
    pub iqn: String,
    /// iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
    #[serde(rename = "iscsiInterface", skip_serializing_if = "Option::is_none")]
    pub iscsi_interface: Option<String>,
    /// iSCSI Target Lun number.
    #[serde(rename = "lun")]
    pub lun: i32,
    /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(rename = "portals", skip_serializing_if = "Option::is_none")]
    pub portals: Option<Vec<String>>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<crate::models::V1LocalObjectReference>,
    /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(rename = "targetPortal")]
    pub target_portal: String,
}

impl V1IscsiVolumeSource {
    /// Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.
    pub fn new(iqn: String, lun: i32, target_portal: String) -> V1IscsiVolumeSource {
        V1IscsiVolumeSource {
            chap_auth_discovery: None,
            chap_auth_session: None,
            fs_type: None,
            initiator_name: None,
            iqn,
            iscsi_interface: None,
            lun,
            portals: None,
            read_only: None,
            secret_ref: None,
            target_portal,
        }
    }
}


