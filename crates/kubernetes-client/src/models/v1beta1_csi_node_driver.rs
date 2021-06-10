/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1CsiNodeDriver : CSINodeDriver holds information about the specification of one CSI driver installed on a node



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1CsiNodeDriver {
    #[serde(rename = "allocatable", skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<Box<crate::models::V1beta1VolumeNodeResources>>,
    /// This is the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.
    #[serde(rename = "name")]
    pub name: String,
    /// nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as \"node1\", but the storage system may refer to the same node as \"nodeA\". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. \"nodeA\" instead of \"node1\". This field is required.
    #[serde(rename = "nodeID")]
    pub node_id: String,
    /// topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. \"company.com/zone\", \"company.com/region\"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.
    #[serde(rename = "topologyKeys", skip_serializing_if = "Option::is_none")]
    pub topology_keys: Option<Vec<String>>,
}

impl V1beta1CsiNodeDriver {
    /// CSINodeDriver holds information about the specification of one CSI driver installed on a node
    pub fn new(name: String, node_id: String) -> V1beta1CsiNodeDriver {
        V1beta1CsiNodeDriver {
            allocatable: None,
            name,
            node_id,
            topology_keys: None,
        }
    }
}


