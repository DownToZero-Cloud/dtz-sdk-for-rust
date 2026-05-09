# AssumeIdentityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> |  | [optional]
**identity_id** | Option<[**dtz_identifier::IdentityId**](dtz_identifier::IdentityId.md)> |  | [optional]
**context_id** | Option<[**dtz_identifier::ContextId**](dtz_identifier::ContextId.md)> | target context the token is issued for. The request fails if the caller is not authorized to assume the identity, the identity does not exist, the context does not exist, or the identity has no access to the context. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


