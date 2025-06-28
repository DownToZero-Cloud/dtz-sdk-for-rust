# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_id** | [**dtz_identifier::ContextId**](dtz_identifier::ContextId.md) |  | 
**domain** | Option<**Vec<String>**> | by default this property is empty, this property is only populated if it was part of the service creation. | [optional]
**service_id** | [**dtz_identifier::ServiceId**](dtz_identifier::ServiceId.md) |  | 
**created** | **String** |  | 
**updated** | Option<**String**> |  | [optional]
**prefix** | **String** |  | 
**container_image** | **String** |  | 
**container_image_version** | Option<**String**> |  | [optional]
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**env_variables** | Option<[**std::collections::HashMap<String, models::JobResponseEnvVariablesValue>**](JobResponse_envVariables_value.md)> |  | [optional]
**rewrite** | Option<[**models::UpdateServiceRequestRewrite**](updateService_request_rewrite.md)> |  | [optional]
**login** | Option<[**models::ServiceLogin**](Service_login.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


