# JobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**dtz_identifier::JobId**](dtz_identifier::JobId.md) |  | 
**name** | **String** |  | 
**container_image** | **String** |  | 
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> |  | [optional]
**schedule_type** | **String** |  | 
**schedule_repeat** | Option<**String**> |  | [optional]
**schedule_cron** | Option<**String**> |  | [optional]
**env_variables** | Option<[**std::collections::HashMap<String, models::JobResponseEnvVariablesValue>**](JobResponse_envVariables_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


