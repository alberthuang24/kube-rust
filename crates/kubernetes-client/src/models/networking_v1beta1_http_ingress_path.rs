/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NetworkingV1beta1HttpIngressPath : HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkingV1beta1HttpIngressPath {
    #[serde(rename = "backend")]
    pub backend: Box<crate::models::NetworkingV1beta1IngressBackend>,
    /// Path is matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional \"path\" part of a URL as defined by RFC 3986. Paths must begin with a '/'. When unspecified, all paths from incoming requests are matched.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// PathType determines the interpretation of the Path matching. PathType can be one of the following values: * Exact: Matches the URL path exactly. * Prefix: Matches based on a URL path prefix split by '/'. Matching is   done on a path element by element basis. A path element refers is the   list of labels in the path split by the '/' separator. A request is a   match for path p if every p is an element-wise prefix of p of the   request path. Note that if the last element of the path is a substring   of the last element in request path, it is not a match (e.g. /foo/bar   matches /foo/bar/baz, but does not match /foo/barbaz). * ImplementationSpecific: Interpretation of the Path matching is up to   the IngressClass. Implementations can treat this as a separate PathType   or treat it identically to Prefix or Exact path types. Implementations are required to support all path types. Defaults to ImplementationSpecific.
    #[serde(rename = "pathType", skip_serializing_if = "Option::is_none")]
    pub path_type: Option<String>,
}

impl NetworkingV1beta1HttpIngressPath {
    /// HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend.
    pub fn new(backend: crate::models::NetworkingV1beta1IngressBackend) -> NetworkingV1beta1HttpIngressPath {
        NetworkingV1beta1HttpIngressPath {
            backend: Box::new(backend),
            path: None,
            path_type: None,
        }
    }
}


