# ApiextensionsV1beta1ServiceReference

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | name is the name of the service. Required | 
**namespace** | **String** | namespace is the namespace of the service. Required | 
**path** | Option<**String**> | path is an optional URL path at which the webhook will be contacted. | [optional]
**port** | Option<**i32**> | port is an optional service port at which the webhook will be contacted. `port` should be a valid port number (1-65535, inclusive). Defaults to 443 for backward compatibility. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


