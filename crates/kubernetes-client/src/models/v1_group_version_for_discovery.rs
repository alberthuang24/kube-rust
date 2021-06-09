/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1GroupVersionForDiscovery : GroupVersion contains the \"group/version\" and \"version\" string of a version. It is made a struct to keep extensibility.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1GroupVersionForDiscovery {
    /// groupVersion specifies the API group and version in the form \"group/version\"
    #[serde(rename = "groupVersion")]
    pub group_version: String,
    /// version specifies the version in the form of \"version\". This is to save the clients the trouble of splitting the GroupVersion.
    #[serde(rename = "version")]
    pub version: String,
}

impl V1GroupVersionForDiscovery {
    /// GroupVersion contains the \"group/version\" and \"version\" string of a version. It is made a struct to keep extensibility.
    pub fn new(group_version: String, version: String) -> V1GroupVersionForDiscovery {
        V1GroupVersionForDiscovery {
            group_version,
            version,
        }
    }
}


