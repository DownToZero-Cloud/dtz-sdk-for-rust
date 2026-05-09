# ObjectMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**key** | **String** |  | 
**size** | **i32** |  | 
**size_compressed** | Option<**i32**> |  | [optional]
**last_modified** | [**chrono::DateTime<chrono::FixedOffset>**](chrono::DateTime<chrono::FixedOffset>.md) |  | 
**last_accessed** | [**chrono::DateTime<chrono::FixedOffset>**](chrono::DateTime<chrono::FixedOffset>.md) |  | 
**metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]
**expiration** | Option<[**chrono::DateTime<chrono::FixedOffset>**](chrono::DateTime<chrono::FixedOffset>.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


