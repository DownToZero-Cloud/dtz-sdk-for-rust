# \DefaultApi

All URIs are relative to *https://dtz.rocks/api/2021-12-09*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_chat**](DefaultApi.md#create_chat) | **POST** /chat | create a new chat by posting a message
[**create_context**](DefaultApi.md#create_context) | **POST** /context | create new context
[**create_ingress**](DefaultApi.md#create_ingress) | **POST** /ingress/{domain}/{uri} | create static content for ingress
[**create_root_ingress**](DefaultApi.md#create_root_ingress) | **POST** /ingress/{domain}/ | create or update ingress
[**create_task**](DefaultApi.md#create_task) | **POST** /task/{task_id} | create task for async execution
[**delete_context**](DefaultApi.md#delete_context) | **DELETE** /context/{context_id} | delete context
[**delete_ingress**](DefaultApi.md#delete_ingress) | **DELETE** /ingress/{domain}/{uri} | delete ingress
[**delete_root_ingress**](DefaultApi.md#delete_root_ingress) | **DELETE** /ingress/{domain}/ | delete ingress
[**enable_service**](DefaultApi.md#enable_service) | **POST** /context/{context_id}/enableService | enable service for context
[**get_chat**](DefaultApi.md#get_chat) | **GET** /chat/{chat_id} | get the full chat timeline
[**get_context**](DefaultApi.md#get_context) | **GET** /context/{context_id} | get context information
[**get_current_context**](DefaultApi.md#get_current_context) | **GET** /context | get current context
[**get_ingress**](DefaultApi.md#get_ingress) | **GET** /ingress/{domain}/{uri} | get ingress for '/' path
[**get_root_ingress**](DefaultApi.md#get_root_ingress) | **GET** /ingress/{domain}/ | get ingress for '/' path
[**get_task_history**](DefaultApi.md#get_task_history) | **GET** /task/{task_id} | get execution history
[**issue_certificate**](DefaultApi.md#issue_certificate) | **POST** /certificate | issue a certificate
[**list_available_contexts**](DefaultApi.md#list_available_contexts) | **GET** /identity/availableContexts | list all avaiable contexts
[**list_chat**](DefaultApi.md#list_chat) | **GET** /chat | list all chat threads for the current context
[**list_ingress**](DefaultApi.md#list_ingress) | **GET** /ingress | list all ingress
[**pull_task_from_queue**](DefaultApi.md#pull_task_from_queue) | **POST** /task | pull one task from the async task queue
[**update_context**](DefaultApi.md#update_context) | **POST** /context/{context_id} | update context
[**update_support_case**](DefaultApi.md#update_support_case) | **POST** /chat/{chat_id} | add a new message to the chat



## create_chat

> models::ChatResponseMessage create_chat(create_chat_request)
create a new chat by posting a message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_chat_request** | Option<[**CreateChatRequest**](CreateChatRequest.md)> | chat request request |  |

### Return type

[**models::ChatResponseMessage**](ChatResponseMessage.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_context

> models::ContextResponse create_context(create_context_request)
create new context

create new context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_context_request** | Option<[**CreateContextRequest**](CreateContextRequest.md)> | create a new context |  |

### Return type

[**models::ContextResponse**](ContextResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ingress

> models::IngressResponse create_ingress(domain, uri, create_ingress_request)
create static content for ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**uri** | **String** | uri | [required] |
**create_ingress_request** | Option<[**CreateIngressRequest**](CreateIngressRequest.md)> | serve static content for ingress |  |

### Return type

[**models::IngressResponse**](IngressResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_root_ingress

> models::IngressResponse create_root_ingress(domain, create_ingress_request)
create or update ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**create_ingress_request** | Option<[**CreateIngressRequest**](CreateIngressRequest.md)> | create or update ingress |  |

### Return type

[**models::IngressResponse**](IngressResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_task

> models::CreateTask200Response create_task(task_id, create_task_request)
create task for async execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **dtz_identifier::TaskId** | task id | [required] |
**create_task_request** | Option<[**CreateTaskRequest**](CreateTaskRequest.md)> | create a new task |  |

### Return type

[**models::CreateTask200Response**](createTask_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_context

> delete_context(context_id)
delete context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **dtz_identifier::ContextId** | context id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ingress

> delete_ingress(domain, uri)
delete ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**uri** | **String** | uri | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_root_ingress

> delete_root_ingress(domain)
delete ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_service

> enable_service(context_id)
enable service for context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **dtz_identifier::ContextId** | context id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat

> models::Chat get_chat(chat_id)
get the full chat timeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | chat id | [required] |

### Return type

[**models::Chat**](Chat.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_context

> models::ContextResponse get_context(context_id)
get context information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **dtz_identifier::ContextId** | context id | [required] |

### Return type

[**models::ContextResponse**](ContextResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_context

> models::ContextResponse get_current_context()
get current context

get current context

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ContextResponse**](ContextResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ingress

> models::IngressResponse get_ingress(domain, uri, scope)
get ingress for '/' path

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**uri** | **String** | uri | [required] |
**scope** | **String** | only include the named scope | [required] |

### Return type

[**models::IngressResponse**](IngressResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_root_ingress

> models::IngressResponse get_root_ingress(domain, scope)
get ingress for '/' path

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | domain name | [required] |
**scope** | **String** | only include the named scope | [required] |

### Return type

[**models::IngressResponse**](IngressResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task_history

> get_task_history(task_id)
get execution history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **dtz_identifier::TaskId** | task id | [required] |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_certificate

> issue_certificate(issue_certificate_request)
issue a certificate

issue a certificate -  if no certificate exists a new one is issued, if a certificate exists a new one will only be issued 3 days before its expiration 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_certificate_request** | Option<[**IssueCertificateRequest**](IssueCertificateRequest.md)> | issue a new certificate |  |

### Return type

 (empty response body)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_contexts

> Vec<models::ContextResponse> list_available_contexts()
list all avaiable contexts

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ContextResponse>**](ContextResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_chat

> Vec<models::ListChat200ResponseInner> list_chat()
list all chat threads for the current context

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ListChat200ResponseInner>**](listChat_200_response_inner.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ingress

> Vec<models::IngressResponse> list_ingress(scope)
list all ingress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<**String**> | only include the named scope |  |

### Return type

[**Vec<models::IngressResponse>**](IngressResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_task_from_queue

> models::PullTaskFromQueue200Response pull_task_from_queue(pull_task_from_queue_request)
pull one task from the async task queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_task_from_queue_request** | Option<[**PullTaskFromQueueRequest**](PullTaskFromQueueRequest.md)> | pulls the next task |  |

### Return type

[**models::PullTaskFromQueue200Response**](pullTaskFromQueue_200_response.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_context

> models::ContextResponse update_context(context_id, create_context_request)
update context

update context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | **dtz_identifier::ContextId** | context id | [required] |
**create_context_request** | Option<[**CreateContextRequest**](CreateContextRequest.md)> | update context |  |

### Return type

[**models::ContextResponse**](ContextResponse.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_support_case

> models::ChatResponseMessage update_support_case(chat_id, create_chat_request)
add a new message to the chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_id** | **String** | chat id | [required] |
**create_chat_request** | Option<[**CreateChatRequest**](CreateChatRequest.md)> | chat post |  |

### Return type

[**models::ChatResponseMessage**](ChatResponseMessage.md)

### Authorization

[dtz_oauth](../README.md#dtz_oauth), [dtz_apikey](../README.md#dtz_apikey), [dtz-cookie](../README.md#dtz-cookie)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

