# \DefaultApi

All URIs are relative to *https://rss2email.dtz.rocks/api/2021-02-01*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enable_service**](DefaultApi.md#enable_service) | **POST** /enable | enable the service
[**rss2email_discover_post**](DefaultApi.md#rss2email_discover_post) | **POST** /rss2email/discover | discover feed on homepage
[**rss2email_feed_feed_id_delete**](DefaultApi.md#rss2email_feed_feed_id_delete) | **DELETE** /rss2email/feed/{feed_id} | delete feed
[**rss2email_feed_feed_id_disable_post**](DefaultApi.md#rss2email_feed_feed_id_disable_post) | **POST** /rss2email/feed/{feed_id}/disable | disable feed
[**rss2email_feed_feed_id_enable_post**](DefaultApi.md#rss2email_feed_feed_id_enable_post) | **POST** /rss2email/feed/{feed_id}/enable | enable feed
[**rss2email_feed_get**](DefaultApi.md#rss2email_feed_get) | **GET** /rss2email/feed | get feed data
[**rss2email_feed_post**](DefaultApi.md#rss2email_feed_post) | **POST** /rss2email/feed | create feed subscription
[**rss2email_profile_cancel_subscription_post**](DefaultApi.md#rss2email_profile_cancel_subscription_post) | **POST** /rss2email/profile/cancelSubscription | cancel current subscription
[**rss2email_profile_get**](DefaultApi.md#rss2email_profile_get) | **GET** /rss2email/profile | get profile
[**stats_get**](DefaultApi.md#stats_get) | **GET** /stats | get service statistics
[**update_feed**](DefaultApi.md#update_feed) | **POST** /rss2email/feed/{feed_id} | update feed



## enable_service

> enable_service()
enable the service

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


## rss2email_discover_post

> models::Rss2emailDiscoverPost200Response rss2email_discover_post(rss2email_discover_post_request)
discover feed on homepage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rss2email_discover_post_request** | Option<[**Rss2emailDiscoverPostRequest**](Rss2emailDiscoverPostRequest.md)> | discover request |  |

### Return type

[**models::Rss2emailDiscoverPost200Response**](_rss2email_discover_post_200_response.md)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rss2email_feed_feed_id_delete

> rss2email_feed_feed_id_delete(feed_id)
delete feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rss2email_feed_feed_id_disable_post

> rss2email_feed_feed_id_disable_post(feed_id)
disable feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rss2email_feed_feed_id_enable_post

> rss2email_feed_feed_id_enable_post(feed_id)
enable feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rss2email_feed_get

> rss2email_feed_get()
get feed data

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


## rss2email_feed_post

> models::Rss2emailFeedPost200Response rss2email_feed_post(rss2email_feed_post_request)
create feed subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rss2email_feed_post_request** | Option<[**Rss2emailFeedPostRequest**](Rss2emailFeedPostRequest.md)> | create feed request |  |

### Return type

[**models::Rss2emailFeedPost200Response**](_rss2email_feed_post_200_response.md)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rss2email_profile_cancel_subscription_post

> rss2email_profile_cancel_subscription_post()
cancel current subscription

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


## rss2email_profile_get

> models::Rss2emailProfileGet200Response rss2email_profile_get()
get profile

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Rss2emailProfileGet200Response**](_rss2email_profile_get_200_response.md)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stats_get

> stats_get()
get service statistics

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


## update_feed

> update_feed(feed_id, rss2email_feed_post200_response)
update feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **uuid::Uuid** | feed id | [required] |
**rss2email_feed_post200_response** | Option<[**Rss2emailFeedPost200Response**](Rss2emailFeedPost200Response.md)> | update feed request |  |

### Return type

 (empty response body)

### Authorization

[context_id](../README.md#context_id), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

