/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CoreV1EndpointPort : EndpointPort is a tuple that describes a single port.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreV1EndpointPort {
    /// The application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and http://www.iana.org/assignments/service-names). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol. This is a beta field that is guarded by the ServiceAppProtocol feature gate and enabled by default.
    #[serde(rename = "appProtocol", skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<String>,
    /// The name of this port.  This must match the 'name' field in the corresponding ServicePort. Must be a DNS_LABEL. Optional only if one port is defined.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The port number of the endpoint.
    #[serde(rename = "port")]
    pub port: i32,
    /// The IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl CoreV1EndpointPort {
    /// EndpointPort is a tuple that describes a single port.
    pub fn new(port: i32) -> CoreV1EndpointPort {
        CoreV1EndpointPort {
            app_protocol: None,
            name: None,
            port,
            protocol: None,
        }
    }
}

