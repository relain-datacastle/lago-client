# FeeObjectAmountDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**graduated_ranges** | Option<[**Vec<models::FeeObjectAmountDetailsGraduatedRangesInner>**](FeeObject_amount_details_graduated_ranges_inner.md)> | Graduated ranges, used for a `graduated` charge model. | [optional]
**graduated_percentage_ranges** | Option<[**Vec<models::FeeObjectAmountDetailsGraduatedPercentageRangesInner>**](FeeObject_amount_details_graduated_percentage_ranges_inner.md)> | Graduated percentage ranges, used for a `graduated_percentage` charge model. | [optional]
**free_units** | Option<**String**> | The quantity of units that are provided free of charge for each billing period in a `package` charge model. | [optional]
**paid_units** | Option<**String**> | The quantity of units that are not provided free of charge for each billing period in a `package` charge model. | [optional]
**per_package_size** | Option<**i32**> | The quantity of units included, defined for Package or Percentage charge model. | [optional]
**per_package_unit_amount** | Option<**String**> | Total amount to charge for received paid_units, defined for Package or Percentage charge model. | [optional]
**units** | Option<**String**> | The total units received in Lago for the Percentage charge model. | [optional]
**free_events** | Option<**i32**> | Total number of free events allowed for the Percentage charge model. | [optional]
**rate** | Option<**String**> | Percentage rate applied for the Percentage charge model. | [optional]
**per_unit_total_amount** | Option<**String**> | Total amount of received units to be charged for the Percentage charge model. | [optional]
**paid_events** | Option<**i32**> | Total number of paid events for the Percentage charge model. | [optional]
**fixed_fee_unit_amount** | Option<**String**> | Fixed fee unit price per received paid_event for the Percentage charge model. | [optional]
**fixed_fee_total_amount** | Option<**String**> | Total amount to charge for received paid_events for the Percentage charge model. | [optional]
**min_max_adjustment_total_amount** | Option<**String**> | Total adjustment amount linked to minimum and maximum spending per transaction for the Percentage charge model. | [optional]
**volume_ranges** | Option<[**Vec<models::FeeObjectAmountDetailsVolumeRangesInner>**](FeeObject_amount_details_volume_ranges_inner.md)> | Volume ranges, used for a `volume` charge model. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


