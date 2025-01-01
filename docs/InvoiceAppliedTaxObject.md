# InvoiceAppliedTaxObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the invoice, created by Lago. | [optional]
**fees_amount_cents** | Option<**i32**> | Fees total amount on which the tax is applied | [optional]
**lago_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the applied tax, created by Lago. | [optional]
**lago_tax_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the tax, created by Lago. | [optional]
**tax_name** | Option<**String**> | Name of the tax. | [optional]
**tax_code** | Option<**String**> | Unique code used to identify the tax associated with the API request. | [optional]
**tax_rate** | Option<**f64**> | The percentage rate of the tax | [optional]
**tax_description** | Option<**String**> | Internal description of the taxe | [optional]
**amount_cents** | Option<**i32**> | Amount of the tax | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | Currency of the tax | [optional]
**created_at** | Option<**String**> | The date and time when the applied tax was created. It is expressed in UTC format according to the ISO 8601 datetime standard. This field provides the timestamp for the exact moment when the applied tax was initially created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


