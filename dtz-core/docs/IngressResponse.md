# IngressResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **String** |  | 
**path_prefix** | **String** |  | 
**realm** | Option<**String**> | owning realm, changes can only be performed within the realm | [optional]
**validity** | Option<[**models::Validity**](Validity.md)> |  | [optional]
**login** | Option<[**models::Login**](Login.md)> |  | [optional]
**rewrite** | Option<[**models::Rewrite**](Rewrite.md)> |  | [optional]
**container** | Option<[**models::Container**](Container.md)> |  | [optional]
**static_content** | Option<[**models::StaticContent**](StaticContent.md)> |  | [optional]
**singleton_processor** | Option<**bool**> |  | [optional]
**source_id** | Option<**String**> | source artifact that this ingress is attached to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


