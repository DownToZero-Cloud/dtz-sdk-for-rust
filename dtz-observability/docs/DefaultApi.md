# \DefaultApi

All URIs are relative to *https://observability.dtz.rocks/api/2021-12-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_log_activity**](DefaultApi.md#get_log_activity) | **GET** /log/activity | get log activity over time
[**get_log_attributes**](DefaultApi.md#get_log_attributes) | **GET** /log/attribute | get log attributes
[**get_logs**](DefaultApi.md#get_logs) | **GET** /log | get logs
[**get_stats**](DefaultApi.md#get_stats) | **GET** /stats | get statistics
[**post_log**](DefaultApi.md#post_log) | **POST** /log/push | push log data
[**post_metric**](DefaultApi.md#post_metric) | **POST** /metric | push metric data
[**post_prometheus**](DefaultApi.md#post_prometheus) | **POST** /prometheus | Push endpoint for prometheus data.
[**query_log_activity**](DefaultApi.md#query_log_activity) | **POST** /log/activity | get log activity over time filtered
[**query_logs**](DefaultApi.md#query_logs) | **POST** /log | query logs



## get_log_activity

> Vec<models::GetLogActivity200ResponseInner> get_log_activity()
get log activity over time

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetLogActivity200ResponseInner>**](getLogActivity_200_response_inner.md)

### Authorization

[dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_attributes

> Vec<models::GetLogAttributes200ResponseInner> get_log_attributes()
get log attributes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetLogAttributes200ResponseInner>**](getLogAttributes_200_response_inner.md)

### Authorization

[dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs

> Vec<models::DtzLogsInner> get_logs()
get logs

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DtzLogsInner>**](DtzLogs_inner.md)

### Authorization

[dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stats

> models::GetStats200Response get_stats()
get statistics

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetStats200Response**](getStats_200_response.md)

### Authorization

[dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_log

> post_log(post_log_request_inner)
push log data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_log_request_inner** | Option<[**Vec<models::PostLogRequestInner>**](postLog_request_inner.md)> | log data |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth](../README.md#dtz_auth)

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
**post_metric_request_inner** | Option<[**Vec<models::PostMetricRequestInner>**](postMetric_request_inner.md)> | metric data |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth](../README.md#dtz_auth)

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

[dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_log_activity

> Vec<models::GetLogActivity200ResponseInner> query_log_activity(query_logs_request)
get log activity over time filtered

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_logs_request** | Option<[**QueryLogsRequest**](QueryLogsRequest.md)> | log query |  |

### Return type

[**Vec<models::GetLogActivity200ResponseInner>**](getLogActivity_200_response_inner.md)

### Authorization

[dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_logs

> Vec<models::DtzLogsInner> query_logs(query_logs_request)
query logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_logs_request** | Option<[**QueryLogsRequest**](QueryLogsRequest.md)> | log query |  |

### Return type

[**Vec<models::DtzLogsInner>**](DtzLogs_inner.md)

### Authorization

[dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

