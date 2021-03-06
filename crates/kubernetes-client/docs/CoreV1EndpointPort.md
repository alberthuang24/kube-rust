# CoreV1EndpointPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_protocol** | Option<**String**> | The application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and http://www.iana.org/assignments/service-names). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol. This is a beta field that is guarded by the ServiceAppProtocol feature gate and enabled by default. | [optional]
**name** | Option<**String**> | The name of this port.  This must match the 'name' field in the corresponding ServicePort. Must be a DNS_LABEL. Optional only if one port is defined. | [optional]
**port** | **i32** | The port number of the endpoint. | 
**protocol** | Option<**String**> | The IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


