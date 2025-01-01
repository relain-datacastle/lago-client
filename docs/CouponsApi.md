# \CouponsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_coupon**](CouponsApi.md#apply_coupon) | **POST** /applied_coupons | Apply a coupon to a customer
[**create_coupon**](CouponsApi.md#create_coupon) | **POST** /coupons | Create a coupon
[**delete_applied_coupon**](CouponsApi.md#delete_applied_coupon) | **DELETE** /customers/{external_customer_id}/applied_coupons/{applied_coupon_id} | Delete an applied coupon
[**destroy_coupon**](CouponsApi.md#destroy_coupon) | **DELETE** /coupons/{code} | Delete a coupon
[**find_all_applied_coupons**](CouponsApi.md#find_all_applied_coupons) | **GET** /applied_coupons | List all applied coupons
[**find_all_coupons**](CouponsApi.md#find_all_coupons) | **GET** /coupons | List all coupons
[**find_coupon**](CouponsApi.md#find_coupon) | **GET** /coupons/{code} | Retrieve a coupon
[**update_coupon**](CouponsApi.md#update_coupon) | **PUT** /coupons/{code} | Update a coupon



## apply_coupon

> models::AppliedCoupon apply_coupon(applied_coupon_input)
Apply a coupon to a customer

This endpoint is used to apply a specific coupon to a customer, before or during a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applied_coupon_input** | [**AppliedCouponInput**](AppliedCouponInput.md) | Apply coupon payload | [required] |

### Return type

[**models::AppliedCoupon**](AppliedCoupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_coupon

> models::Coupon create_coupon(coupon_create_input)
Create a coupon

This endpoint is used to create a coupon that can be then attached to a customer to create a discount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_create_input** | [**CouponCreateInput**](CouponCreateInput.md) | Coupon payload | [required] |

### Return type

[**models::Coupon**](Coupon.md)

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


## destroy_coupon

> models::Coupon destroy_coupon(code)
Delete a coupon

This endpoint is used to delete a coupon.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Unique code used to identify the coupon. | [required] |

### Return type

[**models::Coupon**](Coupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_applied_coupons

> models::AppliedCouponsPaginated find_all_applied_coupons(page, per_page, status, external_customer_id)
List all applied coupons

This endpoint is used to list all applied coupons. You can filter by coupon status and by customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**status** | Option<**String**> | The status of the coupon. Can be either `active` or `terminated`. |  |
**external_customer_id** | Option<**String**> | The customer external unique identifier (provided by your own application) |  |

### Return type

[**models::AppliedCouponsPaginated**](AppliedCouponsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_coupons

> models::CouponsPaginated find_all_coupons(page, per_page)
List all coupons

This endpoint is used to list all existing coupons.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::CouponsPaginated**](CouponsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_coupon

> models::Coupon find_coupon(code)
Retrieve a coupon

This endpoint is used to retrieve a specific coupon.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Unique code used to identify the coupon. | [required] |

### Return type

[**models::Coupon**](Coupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_coupon

> models::Coupon update_coupon(code, coupon_update_input)
Update a coupon

This endpoint is used to update a coupon that can be then attached to a customer to create a discount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Unique code used to identify the coupon. | [required] |
**coupon_update_input** | [**CouponUpdateInput**](CouponUpdateInput.md) | Coupon payload | [required] |

### Return type

[**models::Coupon**](Coupon.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

