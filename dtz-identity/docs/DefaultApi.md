# \DefaultApi

All URIs are relative to *https://identity.dtz.rocks/api/2021-02-21*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_role**](DefaultApi.md#assign_role) | **POST** /me/roles/{roleId} | create role assignment
[**assume_identity**](DefaultApi.md#assume_identity) | **POST** /identity/assume | assume identity
[**authenticate_apikey**](DefaultApi.md#authenticate_apikey) | **POST** /auth/apikey | authenticate with apikey
[**change_authentication**](DefaultApi.md#change_authentication) | **POST** /authentication | update the user authentication, aka change you password
[**check_identity**](DefaultApi.md#check_identity) | **POST** /identity/check | checks whether an identity exists
[**create_api_key**](DefaultApi.md#create_api_key) | **POST** /me/identity/apikey | create api key
[**delete_api_key**](DefaultApi.md#delete_api_key) | **DELETE** /me/identity/apikey/{apikey} | delete api key
[**delete_context_roles**](DefaultApi.md#delete_context_roles) | **DELETE** /context/{context_id} | delete all roles attached to this context
[**delete_identity**](DefaultApi.md#delete_identity) | **DELETE** /me/identity | delete current identity
[**get_abstract_roles**](DefaultApi.md#get_abstract_roles) | **GET** /roles | get roles which are abstract - not assigned to any context or identity
[**get_account_email**](DefaultApi.md#get_account_email) | **GET** /me/email | Get account email
[**get_account_stats**](DefaultApi.md#get_account_stats) | **GET** /me | get account stats
[**get_roles_for_context**](DefaultApi.md#get_roles_for_context) | **GET** /roles/context/{contextId} | get roles for a certain context id
[**get_roles_for_identity**](DefaultApi.md#get_roles_for_identity) | **GET** /roles/identity/{identityId} | get roles for a certain identity id
[**list_authentication**](DefaultApi.md#list_authentication) | **GET** /authentication | list user authentications
[**list_available_contexts**](DefaultApi.md#list_available_contexts) | **GET** /context | get a list of contexts that the user has access to
[**list_identity**](DefaultApi.md#list_identity) | **GET** /identity | get a list of all available identities
[**new_context**](DefaultApi.md#new_context) | **POST** /context/{context_id}/new | create identity requirements for a new context
[**new_identity**](DefaultApi.md#new_identity) | **POST** /identity | creates a new identity
[**oauth_authorize**](DefaultApi.md#oauth_authorize) | **GET** /oauth/authorize | oauth authorize
[**oauth_token**](DefaultApi.md#oauth_token) | **POST** /oauth/token | oauth token request
[**remove_role_assignment**](DefaultApi.md#remove_role_assignment) | **DELETE** /me/roles/{roleId} | remove role assignment from identity
[**share_role**](DefaultApi.md#share_role) | **POST** /roles/context/{contextId}/{roleId}/share | sharing a role with another identity
[**token_refresh**](DefaultApi.md#token_refresh) | **POST** /token/refresh | token refresh
[**user_login**](DefaultApi.md#user_login) | **POST** /token/auth | user login
[**user_signup**](DefaultApi.md#user_signup) | **POST** /signup | create a new identity with the given email as account email, also create an authentication with the given credentials to allow a login, creates a default context



## assign_role

> assign_role(role_id)
create role assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **dtz_identifier::RoleId** | role id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_apikey

> models::TokenResponse authenticate_apikey(apikey_request)
authenticate with apikey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey_request** | Option<[**ApikeyRequest**](ApikeyRequest.md)> |  |  |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_authentication

> change_authentication(change_authentication_request)
update the user authentication, aka change you password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_authentication_request** | Option<[**ChangeAuthenticationRequest**](ChangeAuthenticationRequest.md)> | update an existing authentication |  |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_identity

> models::CheckIdentity200Response check_identity(check_identity_request)
checks whether an identity exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_identity_request** | Option<[**CheckIdentityRequest**](CheckIdentityRequest.md)> | checking for existing identity |  |

### Return type

[**models::CheckIdentity200Response**](checkIdentity_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_key

> String create_api_key(create_api_key_request)
create api key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) | api key creation | [required] |

### Return type

**String**

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(apikey)
delete api key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | api key | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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
**context_id** | **String** | context_id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

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

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_abstract_roles

> models::GetAbstractRoles200Response get_abstract_roles()
get roles which are abstract - not assigned to any context or identity

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAbstractRoles200Response**](getAbstractRoles_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_email

> models::GetAccountEmail200Response get_account_email()
Get account email

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAccountEmail200Response**](getAccountEmail_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_stats

> models::GetAccountStats200Response get_account_stats()
get account stats

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAccountStats200Response**](getAccountStats_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles_for_context

> models::GetRolesForContext200Response get_roles_for_context(context_id)
get roles for a certain context id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **dtz_identifier::ContextId** | context id | [required] |

### Return type

[**models::GetRolesForContext200Response**](getRolesForContext_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles_for_identity

> models::GetRolesForIdentity200Response get_roles_for_identity(identity_id)
get roles for a certain identity id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_id** | **dtz_identifier::IdentityId** | identity id | [required] |

### Return type

[**models::GetRolesForIdentity200Response**](getRolesForIdentity_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_authentication

> models::ListAuthentication200Response list_authentication()
list user authentications

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListAuthentication200Response**](listAuthentication_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_contexts

> Vec<models::ListAvailableContexts200ResponseInner> list_available_contexts()
get a list of contexts that the user has access to

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ListAvailableContexts200ResponseInner>**](listAvailableContexts_200_response_inner.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_identity

> models::ListIdentity200Response list_identity()
get a list of all available identities

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListIdentity200Response**](listIdentity_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_context

> new_context(context_id, new_context_request)
create identity requirements for a new context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **dtz_identifier::ContextId** | context_id | [required] |
**new_context_request** | Option<[**NewContextRequest**](NewContextRequest.md)> | context creation request |  |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## new_identity

> models::ListIdentity200ResponseIdentitiesInner new_identity(new_identity_request)
creates a new identity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_identity_request** | Option<[**NewIdentityRequest**](NewIdentityRequest.md)> | create identity request |  |

### Return type

[**models::ListIdentity200ResponseIdentitiesInner**](listIdentity_200_response_identities_inner.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_authorize

> oauth_authorize(response_type, client_id, redirect_uri, scope, state, nonce)
oauth authorize

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_type** | **String** |  | [required] |
**client_id** | **String** |  | [required] |
**redirect_uri** | **String** |  | [required] |
**scope** | **String** |  | [required] |
**state** | Option<**String**> |  |  |
**nonce** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_token

> models::TokenResponse oauth_token(grant_type, client_id, client_secret, redirect_uri, code)
oauth token request

oauth token request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** |  | [required] |
**client_id** | **String** |  | [required] |
**client_secret** | **String** |  | [required] |
**redirect_uri** | **String** |  | [required] |
**code** | **String** |  | [required] |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_role_assignment

> remove_role_assignment(role_id)
remove role assignment from identity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **dtz_identifier::RoleId** | role id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_role

> share_role(context_id, role_id, check_identity_request)
sharing a role with another identity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **dtz_identifier::ContextId** | context id | [required] |
**role_id** | **dtz_identifier::RoleId** | role id | [required] |
**check_identity_request** | [**CheckIdentityRequest**](CheckIdentityRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_refresh

> models::TokenResponse token_refresh(change_context_request)
token refresh

token refresh

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_context_request** | [**ChangeContextRequest**](ChangeContextRequest.md) |  | [required] |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

