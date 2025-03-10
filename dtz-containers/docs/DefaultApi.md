# \DefaultApi

All URIs are relative to *https://containers.dtz.rocks/api/2021-02-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_domain**](DefaultApi.md#create_domain) | **POST** /domain | create a new domain
[**create_job**](DefaultApi.md#create_job) | **POST** /job | create new job
[**create_service**](DefaultApi.md#create_service) | **POST** /service | create a new service hosting
[**delete_domain**](DefaultApi.md#delete_domain) | **DELETE** /domain/{domain_name} | delete single domain
[**delete_job**](DefaultApi.md#delete_job) | **DELETE** /job/{job_id} | delete single job
[**delete_service**](DefaultApi.md#delete_service) | **DELETE** /service/{serviceId} | delete service
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
[**update_service**](DefaultApi.md#update_service) | **POST** /service/{serviceId} | update service
[**verify_domain**](DefaultApi.md#verify_domain) | **PATCH** /domain/{domain_name} | trigger domain verification



## create_domain

> models::Domain create_domain(create_domain)
create a new domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_domain** | Option<[**CreateDomain**](CreateDomain.md)> | register a new domain within dtz |  |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> models::JobResponse create_job(create_job)
create new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_job** | Option<[**CreateJob**](CreateJob.md)> | update existing hosting |  |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service

> models::Service create_service(create_service)
create a new service hosting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_service** | Option<[**CreateService**](CreateService.md)> | creation request |  |

### Return type

[**models::Service**](Service.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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
**job_id** | **dtz_identifier::JobId** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service

> delete_service(service_id)
delete service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **dtz_identifier::ServiceId** | service id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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
**job_id** | **dtz_identifier::JobId** | uuid of the job | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs

> Vec<models::JobResponse> get_jobs()
list all jobs

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::JobResponse>**](JobResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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
**service_id** | **dtz_identifier::ServiceId** | service id | [required] |

### Return type

[**models::Service**](Service.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> Vec<models::Service> get_services()
get current container services

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Service>**](Service.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_job

> trigger_job(job_id)
trigger single job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **dtz_identifier::JobId** | uuid of the job | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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
**job_id** | **dtz_identifier::JobId** | uuid of the job | [required] |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service

> models::Service update_service(service_id, update_service_request)
update service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **dtz_identifier::ServiceId** | service id | [required] |
**update_service_request** | Option<[**UpdateServiceRequest**](UpdateServiceRequest.md)> | update request |  |

### Return type

[**models::Service**](Service.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_domain

> verify_domain(domain_name)
trigger domain verification

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | name of the domain | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

