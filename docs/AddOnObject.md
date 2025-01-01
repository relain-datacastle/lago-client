# AddOnObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the add-on, created by Lago. | 
**name** | **String** | The name of the add-on. | 
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]
**code** | **String** | Unique code used to identify the add-on. | 
**amount_cents** | **i32** | The cost of the add-on in cents, excluding any applicable taxes, that is billed to a customer. By creating a one-off invoice, you will be able to override this value. | 
**amount_currency** | [**models::Currency**](Currency.md) | The currency of the add-on. | 
**description** | Option<**String**> | The description of the add-on. | [optional]
**created_at** | **String** | The date and time when the add-on was created. It is expressed in UTC format according to the ISO 8601 datetime standard. This field provides the timestamp for the exact moment when the add-on was initially created. | 
**taxes** | Option<[**Vec<models::TaxObject>**](TaxObject.md)> | All taxes applied to the add-on. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


