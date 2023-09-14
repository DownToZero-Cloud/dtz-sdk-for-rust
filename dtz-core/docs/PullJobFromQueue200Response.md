# PullJobFromQueue200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**execution_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**job_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**job_name** | **String** |  | 
**container_image** | **String** |  | 
**container_tag** | Option<**String**> |  | [optional]
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**env_variables** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


