# \DefaultApi

All URIs are relative to *https://dtz.rocks/api/2021-12-09*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_context_id_enable_service_get**](DefaultApi.md#context_context_id_enable_service_get) | **GET** /context/{context_id}/enableService | enable service for context
[**create_context**](DefaultApi.md#create_context) | **POST** /context | create new context
[**create_ingress**](DefaultApi.md#create_ingress) | **POST** /ingress/{domain}/{uri} | create static content for ingress
[**create_job**](DefaultApi.md#create_job) | **POST** /job/{job_id} | create job for async execution
[**create_root_ingress**](DefaultApi.md#create_root_ingress) | **POST** /ingress/{domain}/ | create or update ingress
[**delete_context**](DefaultApi.md#delete_context) | **DELETE** /context/{context_id} | delete context
[**delete_ingress**](DefaultApi.md#delete_ingress) | **DELETE** /ingress/{domain}/{uri} | delete ingress
[**delete_root_ingress**](DefaultApi.md#delete_root_ingress) | **DELETE** /ingress/{domain}/ | delete ingress
[**get_all_context**](DefaultApi.md#get_all_context) | **GET** /context | get all contexts
[**get_context**](DefaultApi.md#get_context) | **GET** /context/{context_id} | get context information
[**get_ingress**](DefaultApi.md#get_ingress) | **GET** /ingress/{domain}/{uri} | get ingress for '/' path
[**get_job_history**](DefaultApi.md#get_job_history) | **GET** /job/{job_id} | get execution history
[**get_root_ingress**](DefaultApi.md#get_root_ingress) | **GET** /ingress/{domain}/ | get ingress for '/' path
[**issue_certificate**](DefaultApi.md#issue_certificate) | **POST** /certificate | issue a certificate, the certificate will only be issued on the first name.
[**list_ingress**](DefaultApi.md#list_ingress) | **GET** /ingress | list all ingress
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

> models::IngressResponse create_ingress(domain, uri, create_ingress_request)
create static content for ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**uri** | **String** | uri | [required] |
**create_ingress_request** | Option<[**CreateIngressRequest**](CreateIngressRequest.md)> | serve static content for ingress |  |

### Return type

[**models::IngressResponse**](IngressResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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


## create_root_ingress

> models::IngressResponse create_root_ingress(domain, create_ingress_request)
create or update ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**create_ingress_request** | Option<[**CreateIngressRequest**](CreateIngressRequest.md)> | create or update ingress |  |

### Return type

[**models::IngressResponse**](IngressResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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


## delete_ingress

> delete_ingress(domain, uri)
delete ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**uri** | **String** | uri | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_root_ingress

> delete_root_ingress(domain)
delete ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |

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

> models::GetAllContext200ResponseContextsInner get_context(context_id)
get context information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | context id | [required] |

### Return type

[**models::GetAllContext200ResponseContextsInner**](getAllContext_200_response_contexts_inner.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ingress

> models::IngressResponse get_ingress(domain, uri)
get ingress for '/' path

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**uri** | **String** | uri | [required] |

### Return type

[**models::IngressResponse**](IngressResponse.md)

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


## get_root_ingress

> models::IngressResponse get_root_ingress(domain)
get ingress for '/' path

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |

### Return type

[**models::IngressResponse**](IngressResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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


## list_ingress

> Vec<models::IngressResponse> list_ingress()
list all ingress

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IngressResponse>**](IngressResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

