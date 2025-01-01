# \TaxesApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tax**](TaxesApi.md#create_tax) | **POST** /taxes | Create a tax
[**destroy_tax**](TaxesApi.md#destroy_tax) | **DELETE** /taxes/{code} | Delete a tax
[**find_all_taxes**](TaxesApi.md#find_all_taxes) | **GET** /taxes | List all taxes
[**find_tax**](TaxesApi.md#find_tax) | **GET** /taxes/{code} | Retrieve a Tax
[**update_tax**](TaxesApi.md#update_tax) | **PUT** /taxes/{code} | Update a tax



## create_tax

> models::Tax create_tax(tax_create_input)
Create a tax

This endpoint creates a new tax representing a customizable tax rate applicable to either the organization or a specific customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tax_create_input** | [**TaxCreateInput**](TaxCreateInput.md) | Tax creation payload | [required] |

### Return type

[**models::Tax**](Tax.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_tax

> models::Tax destroy_tax(code)
Delete a tax

This endpoint is used to delete a tax.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the tax. It serves as a unique identifier associated with a particular tax. The code is typically used for internal or system-level identification purposes. | [required] |

### Return type

[**models::Tax**](Tax.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_taxes

> models::TaxesPaginated find_all_taxes(page, per_page)
List all taxes

This endpoint retrieves all existing taxes representing a customizable tax rate applicable to either the organization or a specific customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |

### Return type

[**models::TaxesPaginated**](TaxesPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_tax

> models::Tax find_tax(code)
Retrieve a Tax

This endpoint retrieves an existing tax representing a customizable tax rate applicable to either the organization or a specific customer. The tax is identified by its unique code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the tax. It serves as a unique identifier associated with a particular tax. The code is typically used for internal or system-level identification purposes. | [required] |

### Return type

[**models::Tax**](Tax.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tax

> models::Tax update_tax(code, tax_update_input)
Update a tax

This endpoint updates an existing tax representing a customizable tax rate applicable to either the organization or a specific customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code of the tax. It serves as a unique identifier associated with a particular tax. The code is typically used for internal or system-level identification purposes. | [required] |
**tax_update_input** | [**TaxUpdateInput**](TaxUpdateInput.md) | Tax update payload | [required] |

### Return type

[**models::Tax**](Tax.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

