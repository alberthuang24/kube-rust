/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1IdRange : IDRange provides a min/max of an allowed range of IDs.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1IdRange {
    /// max is the end of the range, inclusive.
    #[serde(rename = "max")]
    pub max: i64,
    /// min is the start of the range, inclusive.
    #[serde(rename = "min")]
    pub min: i64,
}

impl V1beta1IdRange {
    /// IDRange provides a min/max of an allowed range of IDs.
    pub fn new(max: i64, min: i64) -> V1beta1IdRange {
        V1beta1IdRange {
            max,
            min,
        }
    }
}


