# SubscriptionUpdateInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | If the field is not defined and multiple `active` and `pending` subscriptions exists, Lago will update the `active` subscription. However, if you wish to update a `pending` subscription, please ensure that you include the `status` attribute with the `pending` value in your request body. | [optional]
**subscription** | [**models::SubscriptionUpdateInputSubscription**](SubscriptionUpdateInput_subscription.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


