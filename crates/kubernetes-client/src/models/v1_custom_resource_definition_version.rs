/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1CustomResourceDefinitionVersion : CustomResourceDefinitionVersion describes a version for CRD.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1CustomResourceDefinitionVersion {
    /// additionalPrinterColumns specifies additional columns returned in Table output. See https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables for details. If no columns are specified, a single column displaying the age of the custom resource is used.
    #[serde(rename = "additionalPrinterColumns", skip_serializing_if = "Option::is_none")]
    pub additional_printer_columns: Option<Vec<crate::models::V1CustomResourceColumnDefinition>>,
    /// deprecated indicates this version of the custom resource API is deprecated. When set to true, API requests to this version receive a warning header in the server response. Defaults to false.
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// deprecationWarning overrides the default warning returned to API clients. May only be set when `deprecated` is true. The default warning indicates this version is deprecated and recommends use of the newest served version of equal or greater stability, if one exists.
    #[serde(rename = "deprecationWarning", skip_serializing_if = "Option::is_none")]
    pub deprecation_warning: Option<String>,
    /// name is the version name, e.g. “v1”, “v2beta1”, etc. The custom resources are served under this version at `/apis/<group>/<version>/...` if `served` is true.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<crate::models::V1CustomResourceValidation>,
    /// served is a flag enabling/disabling this version from being served via REST APIs
    #[serde(rename = "served")]
    pub served: bool,
    /// storage indicates this version should be used when persisting custom resources to storage. There must be exactly one version with storage=true.
    #[serde(rename = "storage")]
    pub storage: bool,
    #[serde(rename = "subresources", skip_serializing_if = "Option::is_none")]
    pub subresources: Option<crate::models::V1CustomResourceSubresources>,
}

impl V1CustomResourceDefinitionVersion {
    /// CustomResourceDefinitionVersion describes a version for CRD.
    pub fn new(name: String, served: bool, storage: bool) -> V1CustomResourceDefinitionVersion {
        V1CustomResourceDefinitionVersion {
            additional_printer_columns: None,
            deprecated: None,
            deprecation_warning: None,
            name,
            schema: None,
            served,
            storage,
            subresources: None,
        }
    }
}

