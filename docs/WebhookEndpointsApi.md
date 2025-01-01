# \WebhookEndpointsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook_endpoint**](WebhookEndpointsApi.md#create_webhook_endpoint) | **POST** /webhook_endpoints | Create a webhook_endpoint
[**destroy_webhook_endpoint**](WebhookEndpointsApi.md#destroy_webhook_endpoint) | **DELETE** /webhook_endpoints/{lago_id} | Delete a webhook endpoint
[**find_all_webhook_endpoints**](WebhookEndpointsApi.md#find_all_webhook_endpoints) | **GET** /webhook_endpoints | List all webhook endpoints
[**find_webhook_endpoint**](WebhookEndpointsApi.md#find_webhook_endpoint) | **GET** /webhook_endpoints/{lago_id} | Retrieve a webhook endpoint
[**update_webhook_endpoint**](WebhookEndpointsApi.md#update_webhook_endpoint) | **PUT** /webhook_endpoints/{lago_id} | Update a webhook endpoint



## create_webhook_endpoint

> models::WebhookEndpoint create_webhook_endpoint(webhook_endpoint_create_input)
Create a webhook_endpoint

This endpoint is used to create a webhook endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_endpoint_create_input** | [**WebhookEndpointCreateInput**](WebhookEndpointCreateInput.md) | Webhook Endpoint payload | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_webhook_endpoint

> models::WebhookEndpoint destroy_webhook_endpoint(lago_id)
Delete a webhook endpoint

This endpoint is used to delete an existing webhook endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the webhook endpoint within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the webhook endpoint's record within the Lago system. | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_webhook_endpoints

> models::WebhookEndpointsPaginated find_all_webhook_endpoints(page, per_page)
List all webhook endpoints

This endpoint is used to list all webhook endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::WebhookEndpointsPaginated**](WebhookEndpointsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_webhook_endpoint

> models::WebhookEndpoint find_webhook_endpoint(lago_id)
Retrieve a webhook endpoint

This endpoint is used to retrieve an existing webhook endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the webhook endpoint within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the webhook endpoint's record within the Lago system. | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_endpoint

> models::WebhookEndpoint update_webhook_endpoint(lago_id, webhook_endpoint_update_input)
Update a webhook endpoint

This endpoint is used to update an existing webhook endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the webhook endpoint within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the webhook endpoint's record within the Lago system. | [required] |
**webhook_endpoint_update_input** | [**WebhookEndpointUpdateInput**](WebhookEndpointUpdateInput.md) | Webhook Endpoint update payload | [required] |

### Return type

[**models::WebhookEndpoint**](WebhookEndpoint.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

