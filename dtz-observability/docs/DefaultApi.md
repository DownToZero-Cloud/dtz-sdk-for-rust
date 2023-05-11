# \DefaultApi

All URIs are relative to *https://observability.dtz.rocks/api/2021-12-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logs**](DefaultApi.md#get_logs) | **GET** /log | get logs
[**get_stats**](DefaultApi.md#get_stats) | **GET** /stats | get statistics
[**log_activity_get**](DefaultApi.md#log_activity_get) | **GET** /log/activity | get log activity over time
[**log_activity_post**](DefaultApi.md#log_activity_post) | **POST** /log/activity | get log activity over time filtered
[**log_attribute_get**](DefaultApi.md#log_attribute_get) | **GET** /log/attribute | get log attributes
[**log_push_post**](DefaultApi.md#log_push_post) | **POST** /log/push | push log data
[**post_metric**](DefaultApi.md#post_metric) | **POST** /metric | push metric data
[**post_prometheus**](DefaultApi.md#post_prometheus) | **POST** /prometheus | Push endpoint for prometheus data.
[**query_logs**](DefaultApi.md#query_logs) | **POST** /log | query logs



## get_logs

> get_logs()
get logs

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


## get_stats

> crate::models::GetStats200Response get_stats()
get statistics

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetStats200Response**](getStats_200_response.md)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_activity_get

> Vec<crate::models::LogActivityGet200ResponseInner> log_activity_get()
get log activity over time

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LogActivityGet200ResponseInner>**](_log_activity_get_200_response_inner.md)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_activity_post

> Vec<crate::models::LogActivityGet200ResponseInner> log_activity_post(query_logs_request)
get log activity over time filtered

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_logs_request** | Option<[**QueryLogsRequest**](QueryLogsRequest.md)> | log query |  |

### Return type

[**Vec<crate::models::LogActivityGet200ResponseInner>**](_log_activity_get_200_response_inner.md)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_attribute_get

> Vec<crate::models::LogAttributeGet200ResponseInner> log_attribute_get()
get log attributes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LogAttributeGet200ResponseInner>**](_log_attribute_get_200_response_inner.md)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_push_post

> log_push_post(_log_push_post_request_inner)
push log data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_log_push_post_request_inner** | Option<[**Vec<crate::models::LogPushPostRequestInner>**](_log_push_post_request_inner.md)> | log query |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_metric

> post_metric(post_metric_request_inner)
push metric data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_metric_request_inner** | Option<[**Vec<crate::models::PostMetricRequestInner>**](postMetric_request_inner.md)> | metric data |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_prometheus

> post_prometheus(body)
Push endpoint for prometheus data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**String**> | prometheus format |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_logs

> query_logs(query_logs_request)
query logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_logs_request** | Option<[**QueryLogsRequest**](QueryLogsRequest.md)> | log query |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

