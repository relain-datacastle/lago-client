# WalletUpdateInputWalletRecurringTransactionRulesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique identifier for the recurring transaction rule in the Lago application. Can be used to update a recurring transaction rule. | [optional]
**trigger** | Option<**String**> | The trigger. Possible values are `interval` or `threshold`. | [optional]
**method** | Option<**String**> | The method used for recurring top-up. Possible values are `fixed` or `target`. | [optional]
**interval** | Option<**String**> | The interval used for recurring top-up. It represents the frequency at which automatic top-up occurs. The interval can be one of the following values: `weekly`, `monthly`, `quarterly` or `yearly`. Required only when trigger is set to `interval`. | [optional]
**threshold_credits** | Option<**String**> | The threshold for recurring top-ups is the value at which an automatic top-up is triggered. For example, if this threshold is set at 10 credits, an automatic top-up will occur whenever the wallet balance falls to or below 10 credits. Required only when trigger is set to `threshold`. | [optional]
**paid_credits** | Option<**String**> | The number of paid credits. Required only if there is no granted credits. | [optional]
**granted_credits** | Option<**String**> | The number of free granted credits. Required only if there is no paid credits. | [optional]
**started_at** | Option<**String**> | The effective start date for recurring top-ups. This date should be provided in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | [optional]
**target_ongoing_balance** | Option<**String**> | The target ongoing balance is the value set for the ongoing balance to be reached by the automatic top-up. Required only when trigger is set to `target`. | [optional]
**invoice_requires_successful_payment** | Option<**bool**> | A boolean setting that, when set to true, delays issuing an invoice for a wallet top-up until a successful payment is made; if false, the invoice is issued immediately upon wallet top-up, regardless of the payment status. Default value of false. | [optional]
**transaction_metadata** | Option<[**Vec<models::WalletCreateInputWalletRecurringTransactionRulesInnerTransactionMetadataInner>**](WalletCreateInput_wallet_recurring_transaction_rules_inner_transaction_metadata_inner.md)> | This optional field allows you to store a list of key-value pairs containing additional information or custom attributes. These key-value pairs will populate the metadata of the wallet transactions triggered by this rule. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


