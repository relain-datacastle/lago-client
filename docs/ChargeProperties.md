# ChargeProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**graduated_ranges** | Option<[**Vec<models::ChargePropertiesGraduatedRangesInner>**](ChargeProperties_graduated_ranges_inner.md)> | Graduated ranges, sorted from bottom to top tiers, used for a `graduated` charge model. | [optional]
**graduated_percentage_ranges** | Option<[**Vec<models::ChargePropertiesGraduatedPercentageRangesInner>**](ChargeProperties_graduated_percentage_ranges_inner.md)> | Graduated percentage ranges, sorted from bottom to top tiers, used for a `graduated_percentage` charge model. | [optional]
**amount** | Option<**String**> | - The unit price, excluding tax, for a `standard` charge model. It is expressed as a decimal value. - The amount, excluding tax, for a complete set of units in a `package` charge model. It is expressed as a decimal value. | [optional]
**free_units** | Option<**i32**> | The quantity of units that are provided free of charge for each billing period in a `package` charge model. This field specifies the number of units that customers can use without incurring any additional cost during each billing cycle. | [optional]
**package_size** | Option<**i32**> | The quantity of units included in each pack or set for a `package` charge model. It indicates the number of units that are bundled together as a single package or set within the pricing structure. | [optional]
**rate** | Option<**String**> | The percentage rate that is applied to the amount of each transaction for a `percentage` charge model. It is expressed as a decimal value. | [optional]
**fixed_amount** | Option<**String**> | The fixed fee that is applied to each transaction for a `percentage` charge model. It is expressed as a decimal value. | [optional]
**free_units_per_events** | Option<**i32**> | The count of transactions that are not impacted by the `percentage` rate and fixed fee in a percentage charge model. This field indicates the number of transactions that are exempt from the calculation of charges based on the specified percentage rate and fixed fee. | [optional]
**free_units_per_total_aggregation** | Option<**String**> | The transaction amount that is not impacted by the `percentage` rate and fixed fee in a percentage charge model. This field indicates the portion of the transaction amount that is exempt from the calculation of charges based on the specified percentage rate and fixed fee. | [optional]
**per_transaction_max_amount** | Option<**String**> | Specifies the maximum allowable spending for a single transaction. Working as a transaction cap. | [optional]
**per_transaction_min_amount** | Option<**String**> | Specifies the minimum allowable spending for a single transaction. Working as a transaction floor. | [optional]
**grouped_by** | Option<**Vec<String>**> | The list of event properties that are used to group the events on the invoice for a `standard` charge model. | [optional]
**volume_ranges** | Option<[**Vec<models::ChargePropertiesVolumeRangesInner>**](ChargeProperties_volume_ranges_inner.md)> | Volume ranges, sorted from bottom to top tiers, used for a `volume` charge model. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


