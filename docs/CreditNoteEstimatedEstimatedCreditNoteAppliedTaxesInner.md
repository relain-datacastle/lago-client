# CreditNoteEstimatedEstimatedCreditNoteAppliedTaxesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_tax_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique identifier of the tax, created by Lago. | [optional]
**tax_name** | Option<**String**> | Name of the tax. | [optional]
**tax_code** | Option<**String**> | Unique code used to identify the tax associated with the API request. | [optional]
**tax_rate** | Option<**f64**> | The percentage rate of the tax | [optional]
**tax_description** | Option<**String**> | Internal description of the taxe | [optional]
**base_amount_cents** | Option<**i32**> |  | [optional]
**amount_cents** | Option<**i32**> | Amount of the tax | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | Currency of the tax | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


