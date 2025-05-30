# \DefaultApi

All URIs are relative to *https://billing.dtz.rocks/api/2022-12-28*

Method | HTTP request | Description
------------- | ------------- | -------------
[**charge_stripe_post**](DefaultApi.md#charge_stripe_post) | **POST** /charge/stripe | stripe webhook
[**check_funded**](DefaultApi.md#check_funded) | **GET** /funded | check if identity is funded
[**get_stats**](DefaultApi.md#get_stats) | **GET** /stats | get stats
[**list_transactions**](DefaultApi.md#list_transactions) | **GET** /transaction | get a transaction listing of all incoming and outgoing charges
[**post_consumption**](DefaultApi.md#post_consumption) | **POST** /consumption | post new service consumption



## charge_stripe_post

> charge_stripe_post()
stripe webhook

stripe webhook This endpoint cannot be used with service credentials. Only system credentials can update charge information. 

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


## check_funded

> models::CheckFunded200Response check_funded(identity)
check if identity is funded

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity** | Option<**String**> |  |  |

### Return type

[**models::CheckFunded200Response**](checkFunded_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stats

> models::GetStats200Response get_stats()
get stats

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetStats200Response**](getStats_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transactions

> Vec<models::Transaction> list_transactions(start, end, service, context_id)
get a transaction listing of all incoming and outgoing charges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**String**> | filter timeframe from that date |  |
**end** | Option<**String**> | filter timeframe to that date |  |
**service** | Option<**String**> | filter by service |  |
**context_id** | Option<**String**> | filter by context id |  |

### Return type

[**Vec<models::Transaction>**](Transaction.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_consumption

> post_consumption(consumption)
post new service consumption

post new service consumption This endpoint cannot be used with service credentials. Only system credentials can update service consumption. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consumption** | Option<[**Consumption**](Consumption.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

