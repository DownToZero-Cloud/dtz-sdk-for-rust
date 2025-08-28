# \DefaultApi

All URIs are relative to *https://objectstore.dtz.rocks/api/2022-11-28*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object**](DefaultApi.md#delete_object) | **DELETE** /obj/{objectPath} | Delete object
[**disable_service**](DefaultApi.md#disable_service) | **POST** /disable | disable the objectstore service
[**enable_service**](DefaultApi.md#enable_service) | **POST** /enable | enable the objectstore service
[**get_object**](DefaultApi.md#get_object) | **GET** /obj/{objectPath} | Get Object
[**get_object_metadata**](DefaultApi.md#get_object_metadata) | **HEAD** /obj/{objectPath} | Get Object Metadata
[**list_objects**](DefaultApi.md#list_objects) | **GET** /obj/ | List objects
[**put_object**](DefaultApi.md#put_object) | **PUT** /obj/{objectPath} | Put Object
[**stats**](DefaultApi.md#stats) | **GET** /stats | get service statistics



## delete_object

> delete_object(object_path)
Delete object

This can only be done by the logged in user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_path** | **String** |  | [required] |

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
disable the objectstore service

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


## enable_service

> enable_service()
enable the objectstore service

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


## get_object

> std::path::PathBuf get_object(object_path)
Get Object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_path** | **String** | object patch | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_metadata

> models::ObjectMetadata get_object_metadata(object_path)
Get Object Metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_path** | **String** | object patch | [required] |

### Return type

[**models::ObjectMetadata**](ObjectMetadata.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_objects

> Vec<models::ObjectMetadata> list_objects(prefix)
List objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | Option<**String**> | prefix to search for |  |

### Return type

[**Vec<models::ObjectMetadata>**](ObjectMetadata.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object

> put_object(object_path, x_dtz_expiration, x_dtz_expire_in, x_dtz_expire_at, x_dtz_realm, body)
Put Object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_path** | **String** | object path | [required] |
**x_dtz_expiration** | Option<**String**> | expiration of the object, format is a iso8601 duration,e.g. \"P1D\" for 1 day, \"PT2H\" for 2 hours |  |
**x_dtz_expire_in** | Option<**String**> | expiration of the object, format is a iso8601 duration,e.g. \"P1D\" for 1 day, \"PT2H\" for 2 hours |  |
**x_dtz_expire_at** | Option<**String**> | expiration of the object, format is a rfc3339 timestamp, e.g. \"2025-04-01T13:44:00Z\" |  |
**x_dtz_realm** | Option<**String**> | see docs https://downtozero.cloud/docs e.g. dtz-objectstore |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/octet-stream:
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stats

> models::Stats stats()
get service statistics

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Stats**](Stats.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

