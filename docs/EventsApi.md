# \EventsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_batch_events**](EventsApi.md#create_batch_events) | **POST** /events/batch | Batch multiple events
[**create_event**](EventsApi.md#create_event) | **POST** /events | Send usage events
[**event_estimate_fees**](EventsApi.md#event_estimate_fees) | **POST** /events/estimate_fees | Estimate fees for a pay in advance charge
[**find_all_events**](EventsApi.md#find_all_events) | **GET** /events | List all events
[**find_event**](EventsApi.md#find_event) | **GET** /events/{transaction_id} | Retrieve a specific event



## create_batch_events

> create_batch_events(event_batch_input)
Batch multiple events

This endpoint can be used to send a batch of usage records. Each request may include up to 100 events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_batch_input** | [**EventBatchInput**](EventBatchInput.md) | Batch events payload | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_event

> models::Event create_event(event_input)
Send usage events

This endpoint is used for transmitting usage measurement events to either a designated customer or a specific subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_input** | [**EventInput**](EventInput.md) | Event payload | [required] |

### Return type

[**models::Event**](Event.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_estimate_fees

> models::Fees event_estimate_fees(event_estimate_fees_input)
Estimate fees for a pay in advance charge

Estimate the fees that would be created after reception of an event for a billable metric attached to one or multiple pay in advance charges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_estimate_fees_input** | [**EventEstimateFeesInput**](EventEstimateFeesInput.md) | Event estimate payload | [required] |

### Return type

[**models::Fees**](Fees.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_events

> models::FindAllEvents200Response find_all_events(page, per_page, external_subscription_id, code, timestamp_from, timestamp_to)
List all events

This endpoint is used for retrieving all events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**external_subscription_id** | Option<**String**> | External subscription ID |  |
**code** | Option<**String**> | Filter events by its code. |  |
**timestamp_from** | Option<**String**> | Filter events by timestamp starting from a specific date. |  |
**timestamp_to** | Option<**String**> | Filter events by timestamp up to a specific date. |  |

### Return type

[**models::FindAllEvents200Response**](findAllEvents_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_event

> models::Event find_event(transaction_id)
Retrieve a specific event

This endpoint is used for retrieving a specific usage measurement event that has been sent to a customer or a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | This field represents the unique identifier sent for this specific event (must be URL encoded). | [required] |

### Return type

[**models::Event**](Event.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

