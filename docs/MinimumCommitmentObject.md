# MinimumCommitmentObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the minimum commitment, created by Lago. | 
**plan_code** | Option<**String**> | The unique code representing the plan to be attached to the customer. | [optional]
**amount_cents** | **i32** | The amount of the minimum commitment in cents. | 
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the default name will be used as the display name. | [optional]
**interval** | Option<**String**> | The interval used for recurring billing. It represents the frequency at which subscription billing occurs. The interval can be one of the following values: `yearly`, `quarterly`, `monthly` or `weekly`. | [optional]
**created_at** | **String** | The date and time when the minimum commitment was created. It is expressed in UTC format according to the ISO 8601 datetime standard. This field provides the timestamp for the exact moment when the minimum commitment was initially created. | 
**updated_at** | Option<**String**> | The date and time when the minimum commitment was updated. It is expressed in UTC format according to the ISO 8601 datetime standard. This field provides the timestamp for the exact moment when the minimum commitment was initially created. | [optional]
**taxes** | Option<[**Vec<models::TaxObject>**](TaxObject.md)> | All taxes applied to the minimum commitment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


