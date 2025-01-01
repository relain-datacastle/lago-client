# ChargePropertiesGraduatedRangesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_value** | **i32** | Specifies the lower value of a tier for a `graduated` charge model. It must be either 0 or the previous range's `to_value + 1` to maintain the proper sequence of values. | 
**to_value** | **i32** | Specifies the highest value of a tier for a `graduated` charge model. - This value must be higher than the from_value of the same tier. - This value must be null for the last tier. | 
**flat_amount** | **String** | The flat amount for a whole tier, excluding tax, for a `graduated` charge model. It is expressed as a decimal value. | 
**per_unit_amount** | **String** | The unit price, excluding tax, for a specific tier of a `graduated` charge model. It is expressed as a decimal value. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


