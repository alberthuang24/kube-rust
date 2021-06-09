/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// V1beta1ClusterRoleBinding : ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject. Deprecated in v1.17 in favor of rbac.authorization.k8s.io/v1 ClusterRoleBinding, and will no longer be served in v1.22.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1beta1ClusterRoleBinding {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::V1ObjectMeta>,
    #[serde(rename = "roleRef")]
    pub role_ref: crate::models::V1beta1RoleRef,
    /// Subjects holds references to the objects the role applies to.
    #[serde(rename = "subjects", skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<crate::models::RbacV1beta1Subject>>,
}

impl V1beta1ClusterRoleBinding {
    /// ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject. Deprecated in v1.17 in favor of rbac.authorization.k8s.io/v1 ClusterRoleBinding, and will no longer be served in v1.22.
    pub fn new(role_ref: crate::models::V1beta1RoleRef) -> V1beta1ClusterRoleBinding {
        V1beta1ClusterRoleBinding {
            api_version: None,
            kind: None,
            metadata: None,
            role_ref,
            subjects: None,
        }
    }
}


