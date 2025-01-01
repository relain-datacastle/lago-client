# ChargePropertiesGraduatedPercentageRangesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_value** | **i32** | Specifies the lower value of a tier for a `graduated_percentage` charge model. It must be either 0 or the previous range's `to_value + 1` to maintain the proper sequence of values. | 
**to_value** | **i32** | Specifies the highest value of a tier for a `graduated_percentage` charge model. - This value must be higher than the from_value of the same tier. - This value must be null for the last tier. | 
**rate** | **String** | The percentage rate that is applied to the amount of each transaction in the tier for a `graduated_percentage` charge model. It is expressed as a decimal value. | 
**flat_amount** | **String** | The flat amount for a whole tier, excluding tax, for a `graduated_percentage` charge model. It is expressed as a decimal value. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


