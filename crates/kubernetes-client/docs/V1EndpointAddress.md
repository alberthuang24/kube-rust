# V1EndpointAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hostname** | Option<**String**> | The Hostname of this endpoint | [optional]
**ip** | **String** | The IP of this endpoint. May not be loopback (127.0.0.0/8), link-local (169.254.0.0/16), or link-local multicast ((224.0.0.0/24). IPv6 is also accepted but not fully supported on all platforms. Also, certain kubernetes components, like kube-proxy, are not IPv6 ready. | 
**node_name** | Option<**String**> | Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node. | [optional]
**target_ref** | Option<[**crate::models::V1ObjectReference**](v1.ObjectReference.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


