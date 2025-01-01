# InvoiceOneOffCreateInputInvoiceFeesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_on_code** | **String** | The code of the add-on used as invoice item. | 
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]
**unit_amount_cents** | Option<**i32**> | The amount of the fee per unit, expressed in cents. By default, the amount of the add-on is used. | [optional]
**units** | Option<**String**> | The quantity of units associated with the fee. By default, only 1 unit is added to the invoice. | [optional]
**description** | Option<**String**> | This is a description | [optional]
**tax_codes** | Option<**Vec<String>**> | List of unique code used to identify the taxes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


