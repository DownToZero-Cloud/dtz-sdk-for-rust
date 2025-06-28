# UpdateServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | Option<**Vec<String>**> | by default this property is empty and represents that all verified domains will be added. I a domain is added through a service, this service will only be served through that domain, und new domain als also no longer added automatically. | [optional]
**prefix** | **String** |  | 
**container_image** | **String** |  | 
**container_image_version** | Option<**String**> |  | [optional]
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**env_variables** | Option<[**std::collections::HashMap<String, models::UpdateServiceRequestEnvVariablesValue>**](updateService_request_envVariables_value.md)> |  | [optional]
**rewrite** | Option<[**models::UpdateServiceRequestRewrite**](updateService_request_rewrite.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


