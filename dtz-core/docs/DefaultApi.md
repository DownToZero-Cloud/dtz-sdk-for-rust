# \DefaultApi

All URIs are relative to *https://dtz.rocks/api/2021-12-09*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_context_id_enable_service_get**](DefaultApi.md#context_context_id_enable_service_get) | **GET** /context/{context_id}/enableService | enable service for context
[**create_context**](DefaultApi.md#create_context) | **POST** /context | create new context
[**create_ingress**](DefaultApi.md#create_ingress) | **POST** /ingress | create or update ingress
[**create_ingress_content**](DefaultApi.md#create_ingress_content) | **POST** /ingress/{host}/{uri} | create static content for ingress
[**create_job**](DefaultApi.md#create_job) | **POST** /job/{job_id} | create job for async execution
[**delete_context**](DefaultApi.md#delete_context) | **DELETE** /context/{context_id} | delete context
[**delete_ingress_content**](DefaultApi.md#delete_ingress_content) | **DELETE** /ingress/{host}/{uri} | delete ingress content
[**get_all_context**](DefaultApi.md#get_all_context) | **GET** /context | get all contexts
[**get_context**](DefaultApi.md#get_context) | **GET** /context/{context_id} | get context information
[**get_ingress**](DefaultApi.md#get_ingress) | **GET** /ingress | get all ingress
[**get_job_history**](DefaultApi.md#get_job_history) | **GET** /job/{job_id} | get execution history
[**issue_certificate**](DefaultApi.md#issue_certificate) | **POST** /certificate | issue a certificate, the certificate will only be issued on the first name.
[**pull_job_from_queue**](DefaultApi.md#pull_job_from_queue) | **POST** /job | pull one job from the async job queue



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

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_context

> models::CreateContext200Response create_context(create_context_request)
create new context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_context_request** | Option<[**CreateContextRequest**](CreateContextRequest.md)> | create a new context |  |

### Return type

[**models::CreateContext200Response**](createContext_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ingress

> create_ingress(create_ingress_request)
create or update ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ingress_request** | Option<[**CreateIngressRequest**](CreateIngressRequest.md)> | create or update ingress |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ingress_content

> create_ingress_content(host, uri, create_ingress_content_request)
create static content for ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host** | **String** | host name | [required] |
**uri** | **String** | uri | [required] |
**create_ingress_content_request** | Option<[**CreateIngressContentRequest**](CreateIngressContentRequest.md)> | serve static content for ingress |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

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

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_context

> delete_context(context_id)
delete context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | context id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ingress_content

> delete_ingress_content(host, uri)
delete ingress content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host** | **String** | host name | [required] |
**uri** | **String** | uri | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_context

> models::GetAllContext200Response get_all_context()
get all contexts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAllContext200Response**](getAllContext_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_context

> models::GetContext200Response get_context(context_id)
get context information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | context id | [required] |

### Return type

[**models::GetContext200Response**](getContext_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ingress

> Vec<models::GetIngress200ResponseInner> get_ingress()
get all ingress

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetIngress200ResponseInner>**](getIngress_200_response_inner.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

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

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_job_from_queue

> models::PullJobFromQueue200Response pull_job_from_queue(pull_job_from_queue_request)
pull one job from the async job queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_job_from_queue_request** | Option<[**PullJobFromQueueRequest**](PullJobFromQueueRequest.md)> | pulls the next job |  |

### Return type

[**models::PullJobFromQueue200Response**](pullJobFromQueue_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

