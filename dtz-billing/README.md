# Rust API client for dtz-billing

a generated client for the DTZ Billing API


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `dtz-billing` and add the following to `Cargo.toml` under `[dependencies]`:

```
dtz-billing = { path = "./dtz-billing" }
```

## Documentation for API Endpoints

All URIs are relative to *https://billing.dtz.rocks/api/2022-12-28*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**charge_stripe_post**](docs/DefaultApi.md#charge_stripe_post) | **POST** /charge/stripe | stripe webhook
*DefaultApi* | [**get_stats**](docs/DefaultApi.md#get_stats) | **GET** /stats | get stats
*DefaultApi* | [**post_usage**](docs/DefaultApi.md#post_usage) | **POST** /usage | post new service usage


## Documentation For Models

 - [GetStats200Response](docs/GetStats200Response.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

jens@apimeister.com

