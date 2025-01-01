# \PaymentRequestsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_request**](PaymentRequestsApi.md#create_payment_request) | **POST** /payment_requests | Create a payment request
[**find_all_payment_requests**](PaymentRequestsApi.md#find_all_payment_requests) | **GET** /payment_requests | List all payment requests



## create_payment_request

> models::CreatePaymentRequest200Response create_payment_request(create_payment_request_request)
Create a payment request

This endpoint is used to create a payment request to collect payments of overdue invoices of a given customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_request_request** | [**CreatePaymentRequestRequest**](CreatePaymentRequestRequest.md) | PaymentRequest payload | [required] |

### Return type

[**models::CreatePaymentRequest200Response**](createPaymentRequest_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_payment_requests

> models::PaymentRequestsPaginated find_all_payment_requests(page, per_page, external_customer_id)
List all payment requests

This endpoint is used to list all existing payment requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**external_customer_id** | Option<**String**> | Unique identifier assigned to the customer in your application. |  |

### Return type

[**models::PaymentRequestsPaginated**](PaymentRequestsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

