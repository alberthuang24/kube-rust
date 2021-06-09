/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VersionInfo : Info contains versioning information. how we'll want to distribute that information.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionInfo {
    #[serde(rename = "buildDate")]
    pub build_date: String,
    #[serde(rename = "compiler")]
    pub compiler: String,
    #[serde(rename = "gitCommit")]
    pub git_commit: String,
    #[serde(rename = "gitTreeState")]
    pub git_tree_state: String,
    #[serde(rename = "gitVersion")]
    pub git_version: String,
    #[serde(rename = "goVersion")]
    pub go_version: String,
    #[serde(rename = "major")]
    pub major: String,
    #[serde(rename = "minor")]
    pub minor: String,
    #[serde(rename = "platform")]
    pub platform: String,
}

impl VersionInfo {
    /// Info contains versioning information. how we'll want to distribute that information.
    pub fn new(build_date: String, compiler: String, git_commit: String, git_tree_state: String, git_version: String, go_version: String, major: String, minor: String, platform: String) -> VersionInfo {
        VersionInfo {
            build_date,
            compiler,
            git_commit,
            git_tree_state,
            git_version,
            go_version,
            major,
            minor,
            platform,
        }
    }
}

