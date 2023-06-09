# Rust API client for dtz-identity

a generated client for the DTZ Identity API


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `dtz-identity` and add the following to `Cargo.toml` under `[dependencies]`:

```
dtz-identity = { path = "./dtz-identity" }
```

## Documentation for API Endpoints

All URIs are relative to *https://identity.dtz.rocks/api/2021-02-21*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**assign_role**](docs/DefaultApi.md#assign_role) | **POST** /me/roles/{roleId} | create role assignment
*DefaultApi* | [**auth_apikey_post**](docs/DefaultApi.md#auth_apikey_post) | **POST** /auth/apikey | authenticate with apikey
*DefaultApi* | [**auth_update**](docs/DefaultApi.md#auth_update) | **POST** /authentication/{auth_id} | update an authentication
*DefaultApi* | [**create_api_key**](docs/DefaultApi.md#create_api_key) | **POST** /me/identity/apikey | create api key
*DefaultApi* | [**delete_api_key**](docs/DefaultApi.md#delete_api_key) | **DELETE** /me/identity/apikey/{apikey} | delete api key
*DefaultApi* | [**get_account_stats**](docs/DefaultApi.md#get_account_stats) | **GET** /me | get account stats
*DefaultApi* | [**get_roles**](docs/DefaultApi.md#get_roles) | **GET** /roles | get roles
*DefaultApi* | [**list_auth**](docs/DefaultApi.md#list_auth) | **GET** /authentication | list user authentications
*DefaultApi* | [**remove_role_assignment**](docs/DefaultApi.md#remove_role_assignment) | **DELETE** /me/roles/{roleId} | remove role assignment from identity
*DefaultApi* | [**user_login**](docs/DefaultApi.md#user_login) | **POST** /token/auth | user login
*DefaultApi* | [**user_signup**](docs/DefaultApi.md#user_signup) | **POST** /signup | user signup


## Documentation For Models

 - [AuthApikeyPostRequest](docs/AuthApikeyPostRequest.md)
 - [AuthRequest](docs/AuthRequest.md)
 - [AuthResponse](docs/AuthResponse.md)
 - [AuthUpdateRequest](docs/AuthUpdateRequest.md)
 - [CreateApiKeyRequest](docs/CreateApiKeyRequest.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [GetAccountStats200Response](docs/GetAccountStats200Response.md)
 - [GetAccountStats200ResponseRolesInner](docs/GetAccountStats200ResponseRolesInner.md)
 - [GetRoles200Response](docs/GetRoles200Response.md)
 - [GetRoles200ResponseRolesInner](docs/GetRoles200ResponseRolesInner.md)
 - [SignupRequest](docs/SignupRequest.md)
 - [TokenResponse](docs/TokenResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

jens@apimeister.com

