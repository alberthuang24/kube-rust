# V1ContainerStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_id** | Option<**String**> | Container's ID in the format 'docker://<container_id>'. | [optional]
**image** | **String** | The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images | 
**image_id** | **String** | ImageID of the container's image. | 
**last_state** | Option<[**crate::models::V1ContainerState**](v1.ContainerState.md)> |  | [optional]
**name** | **String** | This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated. | 
**ready** | **bool** | Specifies whether the container has passed its readiness probe. | 
**restart_count** | **i32** | The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC. | 
**started** | Option<**bool**> | Specifies whether the container has passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. Is always true when no startupProbe is defined. | [optional]
**state** | Option<[**crate::models::V1ContainerState**](v1.ContainerState.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

