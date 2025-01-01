# CustomerChargeUsageObjectCharge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the charge within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the charge's record within the Lago system. | 
**charge_model** | **String** | The pricing model applied to this charge. Possible values are standard, `graduated`, `percentage`, `package` or `volume`. | 
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


