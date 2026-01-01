# \DefaultApi

All URIs are relative to *https://containers.dtz.rocks/api/2021-02-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_domain**](DefaultApi.md#create_domain) | **POST** /domain | create a new domain
[**create_job**](DefaultApi.md#create_job) | **POST** /job | create new job
[**create_service**](DefaultApi.md#create_service) | **POST** /service | create a new service hosting
[**create_volume**](DefaultApi.md#create_volume) | **POST** /volume | create a new volume
[**delete_domain**](DefaultApi.md#delete_domain) | **DELETE** /domain/{domainName} | delete single domain
[**delete_job**](DefaultApi.md#delete_job) | **DELETE** /job/{jobId} | delete single job
[**delete_service**](DefaultApi.md#delete_service) | **DELETE** /service/{serviceId} | delete service
[**delete_volume**](DefaultApi.md#delete_volume) | **DELETE** /volume/{volumeId} | delete volume
[**disable**](DefaultApi.md#disable) | **POST** /disable | disable the containers service
[**enable**](DefaultApi.md#enable) | **POST** /enable | enable the containers service
[**get_domain**](DefaultApi.md#get_domain) | **GET** /domain/{domainName} | get single domain
[**get_domains**](DefaultApi.md#get_domains) | **GET** /domain | get all domains
[**get_job**](DefaultApi.md#get_job) | **GET** /job/{jobId} | get single job
[**get_jobs**](DefaultApi.md#get_jobs) | **GET** /job | list all jobs
[**get_service**](DefaultApi.md#get_service) | **GET** /service/{serviceId} | get service
[**get_services**](DefaultApi.md#get_services) | **GET** /service | get current container services
[**get_volume**](DefaultApi.md#get_volume) | **GET** /volume/{volumeId} | get volume
[**get_volume_stats**](DefaultApi.md#get_volume_stats) | **GET** /volume/stats | get stats for all volumes
[**get_volume_stats_for_volume**](DefaultApi.md#get_volume_stats_for_volume) | **GET** /volume/{volumeId}/stats | get stats for a single volume
[**get_volumes**](DefaultApi.md#get_volumes) | **GET** /volume | list volumes
[**trigger_job**](DefaultApi.md#trigger_job) | **PATCH** /job/{jobId} | trigger single job
[**update_job**](DefaultApi.md#update_job) | **POST** /job/{jobId} | update single job
[**update_service**](DefaultApi.md#update_service) | **POST** /service/{serviceId} | update service
[**update_volume**](DefaultApi.md#update_volume) | **POST** /volume/{volumeId} | update volume
[**verify_domain**](DefaultApi.md#verify_domain) | **PATCH** /domain/{domainName} | trigger domain verification



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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> models::JobResponse create_job(create_job_request)
create new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_job_request** | Option<[**CreateJobRequest**](CreateJobRequest.md)> | update existing hosting |  |

### Return type

[**models::JobResponse**](JobResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_volume

> models::Volume create_volume(create_volume_request)
create a new volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_volume_request** | Option<[**CreateVolumeRequest**](CreateVolumeRequest.md)> | create a volume |  |

### Return type

[**models::Volume**](Volume.md)

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


## delete_volume

> delete_volume(volume_id)
delete volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | volume id | [required] |

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
- **Accept**: application/json

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


## get_volume

> models::Volume get_volume(volume_id)
get volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | volume id | [required] |

### Return type

[**models::Volume**](Volume.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume_stats

> Vec<models::VolumeStats> get_volume_stats()
get stats for all volumes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::VolumeStats>**](VolumeStats.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume_stats_for_volume

> models::VolumeStats get_volume_stats_for_volume(volume_id)
get stats for a single volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | volume id | [required] |

### Return type

[**models::VolumeStats**](VolumeStats.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volumes

> Vec<models::Volume> get_volumes()
list volumes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Volume>**](Volume.md)

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


## update_volume

> models::Volume update_volume(volume_id, update_volume_request)
update volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | volume id | [required] |
**update_volume_request** | Option<[**UpdateVolumeRequest**](UpdateVolumeRequest.md)> | update request |  |

### Return type

[**models::Volume**](Volume.md)

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

