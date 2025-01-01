# AddOnBaseInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the add-on. | [optional]
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]
**code** | Option<**String**> | Unique code used to identify the add-on. | [optional]
**amount_cents** | Option<**i32**> | The cost of the add-on in cents, excluding any applicable taxes, that is billed to a customer. By creating a one-off invoice, you will be able to override this value. | [optional]
**amount_currency** | Option<[**models::Currency**](Currency.md)> | The currency of the add-on. | [optional]
**description** | Option<**String**> | The description of the add-on. | [optional]
**tax_codes** | Option<**Vec<String>**> | List of unique code used to identify the taxes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


