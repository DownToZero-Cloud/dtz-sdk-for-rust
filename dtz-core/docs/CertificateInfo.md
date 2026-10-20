# CertificateInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **String** |  | 
**context_id** | [**dtz_identifier::ContextId**](dtz_identifier::ContextId.md) | the context the certificate is stored for | 
**owner** | Option<[**dtz_identifier::ContextId**](dtz_identifier::ContextId.md)> | the context whose ingress serves the host; absent when the host has no ingress. a renewal has to be requested by this context | [optional]
**valid_from** | [**chrono::DateTime<chrono::FixedOffset>**](chrono::DateTime<chrono::FixedOffset>.md) |  | 
**valid_to** | [**chrono::DateTime<chrono::FixedOffset>**](chrono::DateTime<chrono::FixedOffset>.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


