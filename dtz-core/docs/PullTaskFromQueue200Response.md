# PullTaskFromQueue200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_id** | [**dtz_identifier::ContextId**](dtz_identifier::ContextId.md) |  | 
**execution_id** | [**dtz_identifier::ExecutionId**](dtz_identifier::ExecutionId.md) |  | 
**task_id** | [**dtz_identifier::TaskId**](dtz_identifier::TaskId.md) |  | 
**task_name** | **String** |  | 
**container_image** | **String** |  | 
**container_image_version** | Option<**String**> |  | [optional]
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**env_variables** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


