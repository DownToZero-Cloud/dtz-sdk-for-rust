# CreateTaskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_name** | **String** |  | 
**service** | **String** | origin service, like dtz-flows, dtz-containers | 
**earliest_start** | [**chrono::DateTime<chrono::FixedOffset>**](chrono::DateTime<chrono::FixedOffset>.md) |  | 
**latest_start** | [**chrono::DateTime<chrono::FixedOffset>**](chrono::DateTime<chrono::FixedOffset>.md) |  | 
**require_eco_mode** | **bool** |  | 
**task_definition** | [**models::CreateTaskRequestTaskDefinition**](CreateTaskRequest_taskDefinition.md) |  | 
**source_id** | Option<**String**> | source artifact that this ingress is attached to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


