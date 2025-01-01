# \AnalyticsApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_all_gross_revenues**](AnalyticsApi.md#find_all_gross_revenues) | **GET** /analytics/gross_revenue | List gross revenue
[**find_all_invoice_collections**](AnalyticsApi.md#find_all_invoice_collections) | **GET** /analytics/invoice_collection | List of finalized invoices
[**find_all_invoiced_usages**](AnalyticsApi.md#find_all_invoiced_usages) | **GET** /analytics/invoiced_usage | List usage revenue
[**find_all_mrrs**](AnalyticsApi.md#find_all_mrrs) | **GET** /analytics/mrr | List MRR
[**find_all_overdue_balances**](AnalyticsApi.md#find_all_overdue_balances) | **GET** /analytics/overdue_balance | List overdue balance



## find_all_gross_revenues

> models::GrossRevenues find_all_gross_revenues(currency, external_customer_id, months)
List gross revenue

Gross revenue is the sum of monthly `finalized` invoice payments and fees paid in advance that are not invoiceable. This total is calculated after deducting taxes and discounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<[**models::Currency**](.md)> | Currency of revenue analytics. Format must be ISO 4217. |  |
**external_customer_id** | Option<**String**> | The customer external unique identifier (provided by your own application). Use it to filter revenue analytics at the customer level. |  |
**months** | Option<**i32**> | Show data only for given number of months. |  |

### Return type

[**models::GrossRevenues**](GrossRevenues.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_invoice_collections

> models::InvoiceCollections find_all_invoice_collections(currency, months)
List of finalized invoices

Represents a monthly aggregation, detailing both the total count and the cumulative amount of invoices that have been marked as `finalized`. This report sorts invoices categorically based on their `payment_status`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<[**models::Currency**](.md)> | The currency of revenue analytics. Format must be ISO 4217. |  |
**months** | Option<**i32**> | Show data only for given number of months. |  |

### Return type

[**models::InvoiceCollections**](InvoiceCollections.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_invoiced_usages

> models::InvoicedUsages find_all_invoiced_usages(currency, months)
List usage revenue

Reports a monthly analysis focused on the revenue generated from all usage-based fees. It exclusively accounts for revenue that has been formally invoiced. Importantly, this report does not include revenue related to the usage in the current billing period, limiting its scope to previously invoiced amounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<[**models::Currency**](.md)> | The currency of invoiced usage analytics. Format must be ISO 4217. |  |
**months** | Option<**i32**> | Show data only for given number of months. |  |

### Return type

[**models::InvoicedUsages**](InvoicedUsages.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_mrrs

> models::Mrrs find_all_mrrs(currency, months)
List MRR

This endpoint is used to list MRR.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<[**models::Currency**](.md)> | Quantifies the revenue generated from `subscription` fees on a monthly basis. This figure is calculated post-application of applicable taxes and deduction of any applicable discounts. The method of calculation varies based on the subscription billing cycle:  - Revenue from `monthly` subscription invoices is included in the MRR for the month in which the invoice is issued. - Revenue from `quarterly` subscription invoices is distributed evenly over three months. This distribution applies to fees paid in advance (allocated to the next remaining months depending on calendar or anniversary billing) as well as to fees paid in arrears (allocated to the preceding months depending on calendar or anniversary billing). - Revenue from `yearly` subscription invoices is distributed evenly over twelve months. This allocation is applicable for fees paid in advance (spread over the next  remaining months depending on calendar or anniversary billing) and for fees paid in arrears (spread over the previous months depending on calendar or anniversary billing). - Revenue from `weekly` subscription invoices, the total revenue from all invoices issued within a month is summed up. This total is then divided by the number of invoices issued during that month, and the result is multiplied by 4.33, representing the average number of weeks in a month. |  |
**months** | Option<**i32**> | Show data only for given number of months. |  |

### Return type

[**models::Mrrs**](Mrrs.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_overdue_balances

> models::OverdueBalances find_all_overdue_balances(currency, external_customer_id, months)
List overdue balance

Overdue balance is the total amount associated with overdue invoices (invoices with pending or failed payments which are past their due dates).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<[**models::Currency**](.md)> | Currency of revenue analytics. Format must be ISO 4217. |  |
**external_customer_id** | Option<**String**> | The customer external unique identifier (provided by your own application). Use it to filter revenue analytics at the customer level. |  |
**months** | Option<**i32**> | Show data only for given number of months. |  |

### Return type

[**models::OverdueBalances**](OverdueBalances.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

