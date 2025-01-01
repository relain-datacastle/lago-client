# \CustomersApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer**](CustomersApi.md#create_customer) | **POST** /customers | Create a customer
[**delete_applied_coupon**](CustomersApi.md#delete_applied_coupon) | **DELETE** /customers/{external_customer_id}/applied_coupons/{applied_coupon_id} | Delete an applied coupon
[**destroy_customer**](CustomersApi.md#destroy_customer) | **DELETE** /customers/{external_id} | Delete a customer
[**find_all_customer_past_usage**](CustomersApi.md#find_all_customer_past_usage) | **GET** /customers/{external_customer_id}/past_usage | Retrieve customer past usage
[**find_all_customers**](CustomersApi.md#find_all_customers) | **GET** /customers | List all customers
[**find_customer**](CustomersApi.md#find_customer) | **GET** /customers/{external_id} | Retrieve a customer
[**find_customer_current_usage**](CustomersApi.md#find_customer_current_usage) | **GET** /customers/{external_customer_id}/current_usage | Retrieve customer current usage
[**generate_customer_checkout_url**](CustomersApi.md#generate_customer_checkout_url) | **POST** /customers/{external_customer_id}/checkout_url | Generate a Customer Payment Provider Checkout URL
[**get_customer_portal_url**](CustomersApi.md#get_customer_portal_url) | **GET** /customers/{external_customer_id}/portal_url | Get a customer portal URL



## create_customer

> models::Customer create_customer(customer_create_input)
Create a customer

This endpoint creates a new customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_create_input** | [**CustomerCreateInput**](CustomerCreateInput.md) | Customer payload | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_applied_coupon

> models::AppliedCoupon delete_applied_coupon(external_customer_id, applied_coupon_id)
Delete an applied coupon

This endpoint is used to delete a specific coupon that has been applied to a customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application) | [required] |
**applied_coupon_id** | **uuid::Uuid** | Unique identifier of the applied coupon, created by Lago. | [required] |

### Return type

[**models::AppliedCoupon**](AppliedCoupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_customer

> models::Customer destroy_customer(external_id)
Delete a customer

This endpoint deletes an existing customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing customer | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_customer_past_usage

> models::CustomerPastUsage find_all_customer_past_usage(external_customer_id, external_subscription_id, page, per_page, billable_metric_code, periods_count)
Retrieve customer past usage

This endpoint enables the retrieval of the usage-based billing data for a customer within past periods.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application). | [required] |
**external_subscription_id** | **String** | The unique identifier of the subscription within your application. | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**billable_metric_code** | Option<**String**> | Billable metric code filter to apply to the charge usage |  |
**periods_count** | Option<**i32**> | Number of past billing period to returns in the result |  |

### Return type

[**models::CustomerPastUsage**](CustomerPastUsage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_customers

> models::CustomersPaginated find_all_customers(page, per_page)
List all customers

This endpoint retrieves all existing customers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::CustomersPaginated**](CustomersPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_customer

> models::Customer find_customer(external_id)
Retrieve a customer

This endpoint retrieves an existing customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | External ID of the existing customer | [required] |

### Return type

[**models::Customer**](Customer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_customer_current_usage

> models::CustomerUsage find_customer_current_usage(external_customer_id, external_subscription_id)
Retrieve customer current usage

This endpoint enables the retrieval of the usage-based billing data for a customer within the current period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application). | [required] |
**external_subscription_id** | **String** | The unique identifier of the subscription within your application. | [required] |

### Return type

[**models::CustomerUsage**](CustomerUsage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_customer_checkout_url

> models::GenerateCustomerCheckoutUrl200Response generate_customer_checkout_url(external_customer_id)
Generate a Customer Payment Provider Checkout URL

This endpoint regenerates the Payment Provider Checkout URL of a Customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_customer_id** | **String** | The customer external unique identifier (provided by your own application). | [required] |

### Return type

[**models::GenerateCustomerCheckoutUrl200Response**](generateCustomerCheckoutURL_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customer_portal_url

> models::GetCustomerPortalUrl200Response get_customer_portal_url(external_customer_id)
Get a customer portal URL

Retrieves an embeddable link for displaying a customer portal.  This endpoint allows you to fetch the URL that can be embedded to provide customers access to a dedicated portal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_customer_id** | **String** | External ID of the existing customer | [required] |

### Return type

[**models::GetCustomerPortalUrl200Response**](getCustomerPortalUrl_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

