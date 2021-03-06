# V1alpha1ClusterRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation_rule** | Option<[**crate::models::V1alpha1AggregationRule**](v1alpha1.AggregationRule.md)> |  | [optional]
**api_version** | Option<**String**> | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources | [optional]
**kind** | Option<**String**> | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds | [optional]
**metadata** | Option<[**crate::models::V1ObjectMeta**](v1.ObjectMeta.md)> |  | [optional]
**rules** | Option<[**Vec<crate::models::V1alpha1PolicyRule>**](v1alpha1.PolicyRule.md)> | Rules holds all the PolicyRules for this ClusterRole | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


