# \DefaultApi

All URIs are relative to *https://identity.dtz.rocks/api/2021-02-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_apikey_post**](DefaultApi.md#auth_apikey_post) | **POST** /auth/apikey | authenticate with apikey
[**auth_update**](DefaultApi.md#auth_update) | **POST** /authentication/{auth_id} | update an authentication
[**list_auth**](DefaultApi.md#list_auth) | **GET** /authentication | list user authentications
[**user_login**](DefaultApi.md#user_login) | **POST** /token/auth | user login
[**user_signup**](DefaultApi.md#user_signup) | **POST** /signup | user signup



## auth_apikey_post

> crate::models::TokenResponse auth_apikey_post(auth_apikey_post_request)
authenticate with apikey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_apikey_post_request** | Option<[**AuthApikeyPostRequest**](AuthApikeyPostRequest.md)> |  |  |

### Return type

[**crate::models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_update

> auth_update(auth_id, auth_update_request)
update an authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_id** | **String** | authentication id | [required] |
**auth_update_request** | Option<[**AuthUpdateRequest**](AuthUpdateRequest.md)> | update an existing authnetication |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_auth

> list_auth()
list user authentications

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_login

> crate::models::AuthResponse user_login(auth_request)
user login

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_request** | [**AuthRequest**](AuthRequest.md) | login | [required] |

### Return type

[**crate::models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_signup

> user_signup(signup_request)
user signup

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signup_request** | [**SignupRequest**](SignupRequest.md) | signup | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

