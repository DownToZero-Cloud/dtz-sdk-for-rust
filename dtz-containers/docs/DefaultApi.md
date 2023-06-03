# \DefaultApi

All URIs are relative to *https://containers.dtz.rocks/api/2021-02-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_service**](DefaultApi.md#disable_service) | **POST** /disable | disable the containers service
[**enable_service**](DefaultApi.md#enable_service) | **POST** /enable | enable the containers service
[**get_hosting**](DefaultApi.md#get_hosting) | **GET** /service | get current container hosting
[**job_get**](DefaultApi.md#job_get) | **GET** /job | list all jobs
[**job_job_id_delete**](DefaultApi.md#job_job_id_delete) | **DELETE** /job/{job_id} | delete single job
[**job_job_id_get**](DefaultApi.md#job_job_id_get) | **GET** /job/{job_id} | get single job
[**job_job_id_patch**](DefaultApi.md#job_job_id_patch) | **PATCH** /job/{job_id} | trigger single job
[**job_job_id_post**](DefaultApi.md#job_job_id_post) | **POST** /job/{job_id} | update single job
[**job_post**](DefaultApi.md#job_post) | **POST** /job | create new job
[**update_hosting**](DefaultApi.md#update_hosting) | **POST** /service | update current container hosting



## disable_service

> disable_service()
disable the containers service

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_service

> enable_service()
enable the containers service

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hosting

> get_hosting()
get current container hosting

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_get

> job_get()
list all jobs

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_job_id_delete

> job_job_id_delete(job_id)
delete single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_job_id_get

> job_job_id_get(job_id)
get single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_job_id_patch

> job_job_id_patch(job_id)
trigger single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_job_id_post

> job_job_id_post(job_id)
update single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## job_post

> job_post(job_post_request)
create new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_post_request** | Option<[**JobPostRequest**](JobPostRequest.md)> | update existing hosting |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_hosting

> update_hosting(update_hosting_request)
update current container hosting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_hosting_request** | Option<[**UpdateHostingRequest**](UpdateHostingRequest.md)> | update existing hosting |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

