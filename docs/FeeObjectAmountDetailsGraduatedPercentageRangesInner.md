# FeeObjectAmountDetailsGraduatedPercentageRangesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**units** | **String** | Total units received in Lago. | 
**from_value** | **i32** | Lower value of a tier. It is either 0 or the previous range's `to_value + 1`. | 
**to_value** | **i32** | Highest value of a tier. - This value is higher than the from_value of the same tier. - This value is null for the last tier. | 
**flat_unit_amount** | **String** | Flat unit amount within a specified tier. | 
**rate** | **String** | Percentage rate applied within a specified tier. | 
**per_unit_total_amount** | **String** | Total amount of received units to be charged within a specified tier. | 
**total_with_flat_amount** | **String** | Total amount to be charged for a specific tier, taking into account the flat_unit_amount and the per_unit_total_amount. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


