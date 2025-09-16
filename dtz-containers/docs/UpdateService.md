# UpdateService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | whether this service is active and should be propagated to ingress | 
**domain** | **Vec<String>** | by default this property is empty and represents that all verified domains will be added. I a domain is added through a service, this service will only be served through that domain, und new domain als also no longer added automatically. | 
**prefix** | **String** |  | 
**container_image** | **String** |  | 
**container_image_version** | Option<**String**> |  | [optional]
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**env_variables** | [**std::collections::HashMap<String, models::CreateJobEnvVariablesValue>**](CreateJob_envVariables_value.md) |  | 
**rewrite** | Option<[**models::ServiceRewrite**](Service_rewrite.md)> |  | [optional]
**login** | Option<[**models::UpdateServiceLogin**](UpdateService_login.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


