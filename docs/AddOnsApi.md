# \AddOnsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_add_on**](AddOnsApi.md#create_add_on) | **POST** /add_ons | Create an add-on
[**destroy_add_on**](AddOnsApi.md#destroy_add_on) | **DELETE** /add_ons/{code} | Delete an add-on
[**find_add_on**](AddOnsApi.md#find_add_on) | **GET** /add_ons/{code} | Retrieve an add-on
[**find_all_add_ons**](AddOnsApi.md#find_all_add_ons) | **GET** /add_ons | List all add-ons
[**update_add_on**](AddOnsApi.md#update_add_on) | **PUT** /add_ons/{code} | Update an add-on



## create_add_on

> models::AddOn create_add_on(add_on_create_input)
Create an add-on

This endpoint is used to create an add-on that can be then attached to a one-off invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_on_create_input** | [**AddOnCreateInput**](AddOnCreateInput.md) | Add-on payload | [required] |

### Return type

[**models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_add_on

> models::AddOn destroy_add_on(code)
Delete an add-on

This endpoint is used to delete an existing add-on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Unique code used to identify the add-on. | [required] |

### Return type

[**models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_add_on

> models::AddOn find_add_on(code)
Retrieve an add-on

This endpoint is used to retrieve a specific add-on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Unique code used to identify the add-on. | [required] |

### Return type

[**models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_add_ons

> models::AddOnsPaginated find_all_add_ons(page, per_page)
List all add-ons

This endpoint is used to list all existing add-ons.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::AddOnsPaginated**](AddOnsPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_add_on

> models::AddOn update_add_on(code, add_on_update_input)
Update an add-on

This endpoint is used to update an existing add-on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Unique code used to identify the add-on. | [required] |
**add_on_update_input** | [**AddOnUpdateInput**](AddOnUpdateInput.md) | Add-on payload | [required] |

### Return type

[**models::AddOn**](AddOn.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

