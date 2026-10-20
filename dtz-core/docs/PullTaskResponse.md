# PullTaskResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_id** | [**dtz_identifier::ContextId**](dtz_identifier::ContextId.md) |  | 
**execution_id** | [**dtz_identifier::ExecutionId**](dtz_identifier::ExecutionId.md) |  | 
**task_id** | [**dtz_identifier::TaskId**](dtz_identifier::TaskId.md) |  | 
**task_name** | **String** |  | 
**container_image** | **String** |  | 
**container_pull_user** | Option<**String**> |  | [optional]
**container_pull_pwd** | Option<**String**> | write-only, accepted on create and never returned | [optional]
**env_variables** | Option<[**std::collections::HashMap<String, models::EnvironmentVariablesValue>**](EnvironmentVariables_value.md)> |  | [optional]
**mounts** | Option<[**Vec<models::VolumeMount>**](VolumeMount.md)> |  | [optional]
**source_id** | Option<**String**> | source artifact that this ingress is attached to | [optional]
**access_token** | Option<**String**> | the workload token of the task's context for the hour of the job, attached when the puller holds the task/pull scope of the infra context | [optional]
**database_token** | Option<**String**> | the token of the job's database identity, when the task names one | [optional]
**secrets** | Option<**std::collections::HashMap<String, String>**> | the job's parameters, served to the container by the runner, never put into its environment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


