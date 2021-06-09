/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1SecretProjection : Adapts a secret into a projected volume.  The contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1SecretProjection {
    /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::V1KeyToPath>>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl V1SecretProjection {
    /// Adapts a secret into a projected volume.  The contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.
    pub fn new() -> V1SecretProjection {
        V1SecretProjection {
            items: None,
            name: None,
            optional: None,
        }
    }
}


