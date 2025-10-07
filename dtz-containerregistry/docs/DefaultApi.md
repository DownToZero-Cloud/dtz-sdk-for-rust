# \DefaultApi

All URIs are relative to *https://cr.dtz.rocks/api/2023-12-28*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_authentication**](DefaultApi.md#check_authentication) | **GET** /v2/ | check authentication
[**disable_service**](DefaultApi.md#disable_service) | **POST** /disable | disable the container registry service
[**enable_service**](DefaultApi.md#enable_service) | **POST** /enable | enable the container registry service
[**get_catalog**](DefaultApi.md#get_catalog) | **GET** /v2/_catalog | get catalog
[**get_image_tag_manifest**](DefaultApi.md#get_image_tag_manifest) | **GET** /v2/{image}/manifests/{tag} | get manifest for image and tag
[**get_image_tags_list**](DefaultApi.md#get_image_tags_list) | **GET** /v2/{image}/tags/list | get tags list
[**get_stats**](DefaultApi.md#get_stats) | **GET** /stats | get stats



## check_authentication

> check_authentication()
check authentication

check authentication

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_service

> disable_service()
disable the container registry service

disable the container registry service

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_service

> enable_service()
enable the container registry service

enable the container registry service

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_catalog

> models::CatalogResponse get_catalog()
get catalog

get catalog

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CatalogResponse**](CatalogResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_tag_manifest

> models::ManifestResponse get_image_tag_manifest(image, tag)
get manifest for image and tag

get manifest for image and tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **String** |  | [required] |
**tag** | **String** |  | [required] |

### Return type

[**models::ManifestResponse**](ManifestResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image_tags_list

> models::TagsListResponse get_image_tags_list(image)
get tags list

get tags list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | **String** |  | [required] |

### Return type

[**models::TagsListResponse**](TagsListResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stats

> models::StatsResponse get_stats()
get stats

get stats

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StatsResponse**](StatsResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

