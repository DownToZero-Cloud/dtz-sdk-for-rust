# \DefaultApi

All URIs are relative to *https://rss2email.dtz.rocks/api/2021-02-01*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_subscription**](DefaultApi.md#cancel_subscription) | **POST** /rss2email/profile/cancelSubscription | cancel current subscription
[**create_feed**](DefaultApi.md#create_feed) | **POST** /rss2email/feed | create feed subscription
[**delete_feed**](DefaultApi.md#delete_feed) | **DELETE** /rss2email/feed/{feed_id} | delete feed
[**disable_feed**](DefaultApi.md#disable_feed) | **POST** /rss2email/feed/{feed_id}/disable | disable feed
[**discover_feed**](DefaultApi.md#discover_feed) | **POST** /rss2email/discover | discover feed on homepage
[**enable_feed**](DefaultApi.md#enable_feed) | **POST** /rss2email/feed/{feed_id}/enable | enable feed
[**enable_service**](DefaultApi.md#enable_service) | **POST** /enable | enable the service
[**get_feed**](DefaultApi.md#get_feed) | **GET** /rss2email/feed | get feed data
[**get_profile**](DefaultApi.md#get_profile) | **GET** /rss2email/profile | get profile
[**get_stats**](DefaultApi.md#get_stats) | **GET** /stats | get service statistics
[**update_feed**](DefaultApi.md#update_feed) | **POST** /rss2email/feed/{feed_id} | update feed



## cancel_subscription

> cancel_subscription()
cancel current subscription

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


## create_feed

> models::CreateFeed200Response create_feed(create_feed_request)
create feed subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_feed_request** | Option<[**CreateFeedRequest**](CreateFeedRequest.md)> | create feed request |  |

### Return type

[**models::CreateFeed200Response**](createFeed_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feed

> delete_feed(feed_id)
delete feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_feed

> disable_feed(feed_id)
disable feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discover_feed

> models::DiscoverFeed200Response discover_feed(discover_feed_request)
discover feed on homepage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discover_feed_request** | Option<[**DiscoverFeedRequest**](DiscoverFeedRequest.md)> | discover request |  |

### Return type

[**models::DiscoverFeed200Response**](discoverFeed_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_feed

> enable_feed(feed_id)
enable feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_service

> enable_service()
enable the service

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


## get_feed

> get_feed()
get feed data

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


## get_profile

> models::GetProfile200Response get_profile()
get profile

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetProfile200Response**](getProfile_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stats

> get_stats()
get service statistics

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


## update_feed

> update_feed(feed_id, create_feed200_response)
update feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |
**create_feed200_response** | Option<[**CreateFeed200Response**](CreateFeed200Response.md)> | update feed request |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

