# \DefaultApi

All URIs are relative to *https://containers.dtz.rocks/api/2021-02-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_domain**](DefaultApi.md#create_domain) | **POST** /domain | create a new domain
[**create_job**](DefaultApi.md#create_job) | **POST** /job | create new job
[**create_service**](DefaultApi.md#create_service) | **POST** /service | create a new service hosting
[**delete_domain**](DefaultApi.md#delete_domain) | **DELETE** /domain/{domain_name} | delete single domain
[**delete_job**](DefaultApi.md#delete_job) | **DELETE** /job/{job_id} | delete single job
[**disable**](DefaultApi.md#disable) | **POST** /disable | disable the containers service
[**enable**](DefaultApi.md#enable) | **POST** /enable | enable the containers service
[**get_domain**](DefaultApi.md#get_domain) | **GET** /domain/{domain_name} | get single domain
[**get_domains**](DefaultApi.md#get_domains) | **GET** /domain | get all domains
[**get_job**](DefaultApi.md#get_job) | **GET** /job/{job_id} | get single job
[**get_jobs**](DefaultApi.md#get_jobs) | **GET** /job | list all jobs
[**get_service**](DefaultApi.md#get_service) | **GET** /service/{serviceId} | get service
[**get_services**](DefaultApi.md#get_services) | **GET** /service | get current container services
[**trigger_job**](DefaultApi.md#trigger_job) | **PATCH** /job/{job_id} | trigger single job
[**update_job**](DefaultApi.md#update_job) | **POST** /job/{job_id} | update single job



## create_domain

> models::Domain create_domain(create_domain_request)
create a new domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_domain_request** | Option<[**CreateDomainRequest**](CreateDomainRequest.md)> | register a new domain within dtz |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> create_job(create_job_request)
create new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_job_request** | Option<[**CreateJobRequest**](CreateJobRequest.md)> | update existing hosting |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service

> models::Service create_service(create_service_request)
create a new service hosting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_service_request** | Option<[**CreateServiceRequest**](CreateServiceRequest.md)> | creation request |  |

### Return type

[**models::Service**](Service.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain

> delete_domain(domain_name)
delete single domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | name of the domain | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_job

> delete_job(job_id)
delete single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable

> disable()
disable the containers service

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


## enable

> enable()
enable the containers service

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


## get_domain

> models::Domain get_domain(domain_name)
get single domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | name of the domain | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domains

> Vec<models::Domain> get_domains()
get all domains

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Domain>**](Domain.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> models::JobResponse get_job(job_id)
get single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs

> Vec<models::GetJobs200ResponseInner> get_jobs()
list all jobs

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetJobs200ResponseInner>**](getJobs_200_response_inner.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service

> models::Service get_service(service_id)
get service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | service id | [required] |

### Return type

[**models::Service**](Service.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> get_services()
get current container services

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


## trigger_job

> trigger_job(job_id)
trigger single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_job

> models::JobResponse update_job(job_id)
update single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | uuid of the job | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

