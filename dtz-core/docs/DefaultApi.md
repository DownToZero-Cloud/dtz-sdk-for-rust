# \DefaultApi

All URIs are relative to *https://dtz.rocks/api/2021-12-09*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_context_id_enable_service_get**](DefaultApi.md#context_context_id_enable_service_get) | **GET** /context/{context_id}/enableService | enable service for context
[**context_context_id_get**](DefaultApi.md#context_context_id_get) | **GET** /context/{context_id} | get context information
[**create_context**](DefaultApi.md#create_context) | **POST** /context | create new context
[**create_job**](DefaultApi.md#create_job) | **POST** /job/{job_id} | create job for async execution
[**get_all_context**](DefaultApi.md#get_all_context) | **GET** /context | get all contexts
[**get_job_history**](DefaultApi.md#get_job_history) | **GET** /job/{job_id} | get execution history
[**issue_certificate**](DefaultApi.md#issue_certificate) | **POST** /certificate | issue a certificate, the certificate will only be issued on the first name.



## context_context_id_enable_service_get

> context_context_id_enable_service_get(context_id)
enable service for context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | context id | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_context_id_get

> context_context_id_get(context_id)
get context information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | context id | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_context

> create_context(create_context_request)
create new context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_context_request** | Option<[**CreateContextRequest**](CreateContextRequest.md)> | create a new context |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> create_job(job_id, create_job_request)
create job for async execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | job id | [required] |
**create_job_request** | Option<[**CreateJobRequest**](CreateJobRequest.md)> | issue a new certificate |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_context

> get_all_context()
get all contexts

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


## get_job_history

> get_job_history(job_id)
get execution history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | job id | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_certificate

> issue_certificate(issue_certificate_request)
issue a certificate, the certificate will only be issued on the first name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_certificate_request** | Option<[**IssueCertificateRequest**](IssueCertificateRequest.md)> | issue a new certificate |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

