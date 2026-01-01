# UpdateServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | whether this service is active and should be propagated to ingress | 
**domain** | Option<**Vec<String>**> | by default this property is empty and represents that all verified domains will be added. I a domain is added through a service, this service will only be served through that domain, und new domain als also no longer added automatically. | [optional]
**prefix** | **String** |  | 
**container_image** | **String** |  | 
**container_image_version** | Option<**String**> | the version of the container image; either empty string to reset or a sha256 digest in the form of \"sha256:digest\" | [optional]
**container_port** | Option<**i32**> | Optional port to expose externally; when omitted the first open container port is exposed automatically. | [optional]
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**env_variables** | [**std::collections::HashMap<String, models::CreateJobRequestEnvVariablesValue>**](CreateJobRequest_envVariables_value.md) |  | 
**rewrite** | Option<[**models::ServiceRewrite**](Service_rewrite.md)> |  | [optional]
**login** | Option<[**models::UpdateServiceRequestLogin**](UpdateServiceRequest_login.md)> |  | [optional]
**mounts** | Option<[**Vec<models::VolumeMount>**](VolumeMount.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


