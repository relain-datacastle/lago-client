# \CreditNotesApi

All URIs are relative to *https://api.getlago.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_credit_note**](CreditNotesApi.md#create_credit_note) | **POST** /credit_notes | Create a credit note
[**download_credit_note**](CreditNotesApi.md#download_credit_note) | **POST** /credit_notes/{lago_id}/download | Download a credit note PDF
[**estimate_credit_note**](CreditNotesApi.md#estimate_credit_note) | **POST** /credit_notes/estimate | Estimate amounts for a new credit note
[**find_all_credit_notes**](CreditNotesApi.md#find_all_credit_notes) | **GET** /credit_notes | List all credit notes
[**find_credit_note**](CreditNotesApi.md#find_credit_note) | **GET** /credit_notes/{lago_id} | Retrieve a credit note
[**update_credit_note**](CreditNotesApi.md#update_credit_note) | **PUT** /credit_notes/{lago_id} | Update a credit note
[**void_credit_note**](CreditNotesApi.md#void_credit_note) | **PUT** /credit_notes/{lago_id}/void | Void available credit



## create_credit_note

> models::CreditNote create_credit_note(credit_note_create_input)
Create a credit note

This endpoint creates a new credit note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credit_note_create_input** | [**CreditNoteCreateInput**](CreditNoteCreateInput.md) | Credit note payload | [required] |

### Return type

[**models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_credit_note

> models::CreditNote download_credit_note(lago_id)
Download a credit note PDF

This endpoint downloads the PDF of an existing credit note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | The credit note unique identifier, created by Lago. | [required] |

### Return type

[**models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## estimate_credit_note

> models::CreditNoteEstimated estimate_credit_note(credit_note_estimate_input)
Estimate amounts for a new credit note

This endpoint allows you to retrieve amounts for a new credit note creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credit_note_estimate_input** | Option<[**CreditNoteEstimateInput**](CreditNoteEstimateInput.md)> | Credit note estimate payload |  |

### Return type

[**models::CreditNoteEstimated**](CreditNoteEstimated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_all_credit_notes

> models::CreditNotes find_all_credit_notes(page, per_page, external_customer_id, issuing_date_from, issuing_date_to, search_term, currency, reason, credit_status, refund_status, invoice_number, amount_from, amount_to)
List all credit notes

This endpoint list all existing credit notes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Number of records per page. |  |
**external_customer_id** | Option<**String**> | Unique identifier assigned to the customer in your application. |  |
**issuing_date_from** | Option<**String**> | Filter credit notes starting from a specific date. |  |
**issuing_date_to** | Option<**String**> | Filter credit notes up to a specific date. |  |
**search_term** | Option<**String**> | Search credit notes by id, number, customer name, customer external_id or customer email. |  |
**currency** | Option<**String**> | Filter credit notes by currency. Possible values ISO 4217 currency codes. |  |
**reason** | Option<**String**> | Filter credit notes by reasons. Possible values are `product_unsatisfactory`, `order_change`, `order_cancellation`, `fraudulent_charge`, `duplicated_charge` or `other`. |  |
**credit_status** | Option<**String**> | Filter credit notes by credit status. Possible values are `available`, `consumed`  or `voided`. |  |
**refund_status** | Option<**String**> | Filter credit notes by refund status. Possible values are `pending`, `succeeded`  or `failed`. |  |
**invoice_number** | Option<**String**> | Filter credit notes by their related invoice number. |  |
**amount_from** | Option<**i32**> | Filter credit notes of at least a specific amount. This parameter must be defined in cents to ensure consistent handling for all currency types. |  |
**amount_to** | Option<**i32**> | Filter credit notes up to a specific amount. This parameter must be defined in cents to ensure consistent handling for all currency types. |  |

### Return type

[**models::CreditNotes**](CreditNotes.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_credit_note

> models::CreditNote find_credit_note(lago_id)
Retrieve a credit note

This endpoint retrieves an existing credit note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **String** | The credit note unique identifier, created by Lago. | [required] |

### Return type

[**models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_credit_note

> models::CreditNote update_credit_note(lago_id, credit_note_update_input)
Update a credit note

This endpoint updates an existing credit note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **String** | The credit note unique identifier, created by Lago. | [required] |
**credit_note_update_input** | [**CreditNoteUpdateInput**](CreditNoteUpdateInput.md) | Credit note update payload | [required] |

### Return type

[**models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## void_credit_note

> models::CreditNote void_credit_note(lago_id)
Void available credit

This endpoint voids the available credit linked to a specific credit note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lago_id** | **uuid::Uuid** | The credit note unique identifier, created by Lago. | [required] |

### Return type

[**models::CreditNote**](CreditNote.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

