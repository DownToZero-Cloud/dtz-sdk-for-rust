# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**service_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**created** | **String** |  | 
**prefix** | **String** |  | 
**container_image** | **String** |  | 
**container_image_version** | Option<**String**> |  | [optional]
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**env_variables** | Option<[**serde_json::Value**](.md)> |  | [optional]
**login** | Option<[**models::ServiceLogin**](Service_login.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


