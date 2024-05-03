# \DefaultApi

All URIs are relative to *https://billing.dtz.rocks/api/2022-12-28*

Method | HTTP request | Description
------------- | ------------- | -------------
[**charge_stripe_post**](DefaultApi.md#charge_stripe_post) | **POST** /charge/stripe | stripe webhook
[**get_stats**](DefaultApi.md#get_stats) | **GET** /stats | get stats
[**post_usage**](DefaultApi.md#post_usage) | **POST** /usage | post new service usage



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


## post_usage

> post_usage()
post new service usage

post new service usage This endpoint cannot be used with service credentials. Only system credentials can update service usage. 

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

