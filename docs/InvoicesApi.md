# \InvoicesApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_invoice**](InvoicesApi.md#create_invoice) | **POST** /invoices | Create a one-off invoice
[**download_invoice**](InvoicesApi.md#download_invoice) | **POST** /invoices/{lago_id}/download | Download an invoice PDF
[**finalize_invoice**](InvoicesApi.md#finalize_invoice) | **PUT** /invoices/{lago_id}/finalize | Finalize a draft invoice
[**find_all_invoices**](InvoicesApi.md#find_all_invoices) | **GET** /invoices | List all invoices
[**find_invoice**](InvoicesApi.md#find_invoice) | **GET** /invoices/{lago_id} | Retrieve an invoice
[**invoice_payment_url**](InvoicesApi.md#invoice_payment_url) | **POST** /invoices/{lago_id}/payment_url | Generate a payment URL
[**lose_dispute_invoice**](InvoicesApi.md#lose_dispute_invoice) | **POST** /invoices/{lago_id}/lose_dispute | Mark an invoice payment dispute as lost
[**refresh_invoice**](InvoicesApi.md#refresh_invoice) | **PUT** /invoices/{lago_id}/refresh | Refresh a draft invoice
[**retry_invoice**](InvoicesApi.md#retry_invoice) | **POST** /invoices/{lago_id}/retry | Retry generation of a failed invoice
[**retry_payment**](InvoicesApi.md#retry_payment) | **POST** /invoices/{lago_id}/retry_payment | Retry an invoice payment
[**update_invoice**](InvoicesApi.md#update_invoice) | **PUT** /invoices/{lago_id} | Update an invoice
[**void_invoice**](InvoicesApi.md#void_invoice) | **POST** /invoices/{lago_id}/void | Void an invoice



## create_invoice

> models::Invoice create_invoice(invoice_one_off_create_input)
Create a one-off invoice

This endpoint is used for issuing a one-off invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_one_off_create_input** | [**InvoiceOneOffCreateInput**](InvoiceOneOffCreateInput.md) | Invoice payload | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_invoice

> models::Invoice download_invoice(lago_id)
Download an invoice PDF

This endpoint is used for downloading a specific invoice PDF document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## finalize_invoice

> models::Invoice finalize_invoice(lago_id)
Finalize a draft invoice

This endpoint is used for finalizing a draft invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_invoices

> models::InvoicesPaginated find_all_invoices(page, per_page, external_customer_id, issuing_date_from, issuing_date_to, status, payment_status, payment_overdue, search_term, currency, payment_dispute_lost, invoice_type)
List all invoices

This endpoint is used for retrieving all invoices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**external_customer_id** | Option<**String**> | Unique identifier assigned to the customer in your application. |  |
**issuing_date_from** | Option<**String**> | Filter invoices starting from a specific date. |  |
**issuing_date_to** | Option<**String**> | Filter invoices up to a specific date. |  |
**status** | Option<**String**> | Filter invoices by status. Possible values are `draft` or `finalized`. |  |
**payment_status** | Option<**String**> | Filter invoices by payment status. Possible values are `pending`, `failed` or `succeeded`. |  |
**payment_overdue** | Option<**bool**> | Filter invoices by payment_overdue. Possible values are `true` or `false`. |  |
**search_term** | Option<**String**> | Search invoices by id, number, customer name, customer external_id or customer email. |  |
**currency** | Option<**String**> | Filter invoices by currency. Possible values ISO 4217 currency codes. |  |
**payment_dispute_lost** | Option<**bool**> | Filter invoices with a payment dispute lost. Possible values are `true` or `false`. |  |
**invoice_type** | Option<**String**> | Filter invoices by invoice type. Possible values are `subscription`, `add_on`, `credit`, `one_off` or `advance_charges` |  |

### Return type

[**models::InvoicesPaginated**](InvoicesPaginated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_invoice

> models::Invoice find_invoice(lago_id)
Retrieve an invoice

This endpoint is used for retrieving a specific invoice that has been issued.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoice_payment_url

> models::InvoicePaymentUrl200Response invoice_payment_url(lago_id)
Generate a payment URL

This endpoint generates a checkout link for a specific invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::InvoicePaymentUrl200Response**](invoicePaymentUrl_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lose_dispute_invoice

> models::Invoice lose_dispute_invoice(lago_id)
Mark an invoice payment dispute as lost

This endpoint is used for setting invoice's payment dispute as lost.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_invoice

> models::Invoice refresh_invoice(lago_id)
Refresh a draft invoice

This endpoint is used for refreshing a draft invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retry_invoice

> models::Invoice retry_invoice(lago_id)
Retry generation of a failed invoice

This endpoint is used for retrying to generate a failed invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retry_payment

> retry_payment(lago_id)
Retry an invoice payment

This endpoint resends an invoice for collection and retry a payment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_invoice

> models::Invoice update_invoice(lago_id, invoice_update_input)
Update an invoice

This endpoint is used for updating an existing invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |
**invoice_update_input** | [**InvoiceUpdateInput**](InvoiceUpdateInput.md) | Update an existing invoice | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## void_invoice

> models::Invoice void_invoice(lago_id)
Void an invoice

This endpoint is used for voiding an invoice. You can void an invoice only when it's in a `finalized` status, and the payment status is not `succeeded`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | Unique identifier assigned to the invoice within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the invoice's record within the Lago system. | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

