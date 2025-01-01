# \FeesApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_fee**](FeesApi.md#delete_fee) | **DELETE** /fees/{lago_id} | Delete a fee
[**find_all_fees**](FeesApi.md#find_all_fees) | **GET** /fees | List all fees
[**find_fee**](FeesApi.md#find_fee) | **GET** /fees/{lago_id} | Retrieve a specific fee
[**update_fee**](FeesApi.md#update_fee) | **PUT** /fees/{lago_id} | Update a fee



## delete_fee

> models::Fee delete_fee(lago_id)
Delete a fee

This endpoint is used for deleting a specific fee that has not yet been invoiced

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the fee within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the fee's record within the Lago system. | [required] |

### Return type

[**models::Fee**](Fee.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_fees

> models::FeesPaginated find_all_fees(page, per_page, external_customer_id, external_subscription_id, event_transaction_id, currency, fee_type, billable_metric_code, payment_status, created_at_from, created_at_to, succeeded_at_from, succeeded_at_to, failed_at_from, failed_at_to, refunded_at_from, refunded_at_to)
List all fees

This endpoint is used for retrieving all fees that has been issued.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**external_customer_id** | Option<**String**> | Unique identifier assigned to the customer in your application. |  |
**external_subscription_id** | Option<**String**> | External subscription ID |  |
**event_transaction_id** | Option<**String**> | Filter results by event transaction ID |  |
**currency** | Option<[**models::Currency**](.md)> | Filter results by fee's currency. |  |
**fee_type** | Option<**String**> | The fee type. Possible values are `add-on`, `charge`, `credit` or `subscription`. |  |
**billable_metric_code** | Option<**String**> | Filter results by the `code` of the billable metric attached to the fee. Only applies to `charge` types. |  |
**payment_status** | Option<**String**> | Indicates the payment status of the fee. It represents the current status of the payment associated with the fee. The possible values for this field are `pending`, `succeeded`, `failed` and refunded`. |  |
**created_at_from** | Option<**String**> | Filter results created after creation date and time in UTC. |  |
**created_at_to** | Option<**String**> | Filter results created before creation date and time in UTC. |  |
**succeeded_at_from** | Option<**String**> | Filter results with payment success after creation date and time in UTC. |  |
**succeeded_at_to** | Option<**String**> | Filter results with payment success after creation date and time in UTC. |  |
**failed_at_from** | Option<**String**> | Filter results with payment failure after creation date and time in UTC. |  |
**failed_at_to** | Option<**String**> | Filter results with payment failure after creation date and time in UTC. |  |
**refunded_at_from** | Option<**String**> | Filter results with payment refund after creation date and time in UTC. |  |
**refunded_at_to** | Option<**String**> | Filter results with payment refund after creation date and time in UTC. |  |

### Return type

[**models::FeesPaginated**](FeesPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_fee

> models::Fee find_fee(lago_id)
Retrieve a specific fee

This endpoint is used for retrieving a specific fee that has been issued.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the fee within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the fee's record within the Lago system. | [required] |

### Return type

[**models::Fee**](Fee.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_fee

> models::Fee update_fee(lago_id, fee_update_input)
Update a fee

This endpoint is used for updating a specific fee that has been issued.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the fee within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the fee's record within the Lago system. | [required] |
**fee_update_input** | Option<[**FeeUpdateInput**](FeeUpdateInput.md)> | Fee payload |  |

### Return type

[**models::Fee**](Fee.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

