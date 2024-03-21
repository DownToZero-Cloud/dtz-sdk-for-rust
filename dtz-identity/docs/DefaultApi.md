# \DefaultApi

All URIs are relative to *https://identity.dtz.rocks/api/2021-02-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_role**](DefaultApi.md#assign_role) | **POST** /me/roles/{roleId} | create role assignment
[**assume_identity**](DefaultApi.md#assume_identity) | **POST** /identity/assume | assume identity
[**auth_update**](DefaultApi.md#auth_update) | **POST** /authentication/{auth_id} | update an authentication
[**authenticate_apikey**](DefaultApi.md#authenticate_apikey) | **POST** /auth/apikey | authenticate with apikey
[**create_api_key**](DefaultApi.md#create_api_key) | **POST** /me/identity/apikey | create api key
[**delete_api_key**](DefaultApi.md#delete_api_key) | **DELETE** /me/identity/apikey/{apikey} | delete api key
[**delete_context_roles**](DefaultApi.md#delete_context_roles) | **DELETE** /context/{context_id} | delete all roles attached to this context
[**delete_identity**](DefaultApi.md#delete_identity) | **DELETE** /me/identity | delete current identity
[**get_account_stats**](DefaultApi.md#get_account_stats) | **GET** /me | get account stats
[**get_roles**](DefaultApi.md#get_roles) | **GET** /roles | get roles
[**list_auth**](DefaultApi.md#list_auth) | **GET** /authentication | list user authentications
[**new_context**](DefaultApi.md#new_context) | **POST** /context/{context_id}/new | create identity requirements for a new context
[**remove_role_assignment**](DefaultApi.md#remove_role_assignment) | **DELETE** /me/roles/{roleId} | remove role assignment from identity
[**user_login**](DefaultApi.md#user_login) | **POST** /token/auth | user login
[**user_signup**](DefaultApi.md#user_signup) | **POST** /signup | create a new identity with the given email as account email, also create an authentication with the given credentials to allow a login, creates a default context



## assign_role

> assign_role(role_id)
create role assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **uuid::Uuid** | role id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assume_identity

> models::TokenResponse assume_identity(assume_identity_request)
assume identity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assume_identity_request** | Option<[**AssumeIdentityRequest**](AssumeIdentityRequest.md)> | assume different identity |  |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

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

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_apikey

> models::TokenResponse authenticate_apikey(authenticate_apikey_request)
authenticate with apikey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticate_apikey_request** | Option<[**AuthenticateApikeyRequest**](AuthenticateApikeyRequest.md)> |  |  |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_key

> create_api_key(create_api_key_request)
create api key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) | api key creation | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(apikey)
delete api key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **uuid::Uuid** | api key | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_context_roles

> delete_context_roles(context_id)
delete all roles attached to this context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | context_id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_identity

> delete_identity()
delete current identity

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


## get_account_stats

> models::GetAccountStats200Response get_account_stats()
get account stats

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAccountStats200Response**](getAccountStats_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles

> models::GetRoles200Response get_roles()
get roles

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetRoles200Response**](getRoles_200_response.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_auth

> list_auth()
list user authentications

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


## new_context

> new_context(context_id, new_context_request)
create identity requirements for a new context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **uuid::Uuid** | context_id | [required] |
**new_context_request** | Option<[**NewContextRequest**](NewContextRequest.md)> | context creation request |  |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_role_assignment

> remove_role_assignment(role_id)
remove role assignment from identity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **uuid::Uuid** | role id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_login

> models::TokenResponse user_login(auth_request)
user login

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_request** | [**AuthRequest**](AuthRequest.md) | login | [required] |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_signup

> models::TokenResponse user_signup(signup_request)
create a new identity with the given email as account email, also create an authentication with the given credentials to allow a login, creates a default context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signup_request** | [**SignupRequest**](SignupRequest.md) | signup | [required] |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

[dtz_auth2](../README.md#dtz_auth2), [dtz_auth](../README.md#dtz_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

