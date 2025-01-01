# WalletTransactionObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the wallet transaction within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet transaction's record within the Lago system. | 
**lago_wallet_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the wallet within the Lago application. This ID is exclusively created by Lago and serves as a unique identifier for the wallet's record within the Lago system. | 
**status** | **String** | The status of the wallet transaction. Possible values are `pending` or `settled`. | 
**source** | Option<**String**> | The source field represents the origin or trigger of the wallet transaction. Possible values are `manual`, `interval`. `threshold` | [optional]
**transaction_status** | **String** | The transaction status of the wallet transaction. Possible values are `purchased` (with pending or settled status), `granted` (without invoice_id), `voided` or `invoiced`. | 
**transaction_type** | **String** | The type of transaction. Possible values are `inbound` (increasing the balance) or `outbound` (decreasing the balance). | 
**amount** | **String** | The amount of credits based on the rate and the currency. | 
**credit_amount** | **String** | The number of credits used in the wallet transaction. | 
**invoice_requires_successful_payment** | Option<**bool**> | A boolean setting that, when set to true, delays issuing an invoice for a wallet top-up until a successful payment is made; if false, the invoice is issued immediately upon wallet top-up, regardless of the payment status. Default value of false. | [optional]
**metadata** | Option<[**Vec<models::WalletCreateInputWalletRecurringTransactionRulesInnerTransactionMetadataInner>**](WalletCreateInput_wallet_recurring_transaction_rules_inner_transaction_metadata_inner.md)> | This field allows you to store a list of key-value pairs that hold additional information or custom attributes related to the data. | [optional]
**settled_at** | Option<**String**> | The date when wallet transaction is settled, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | [optional]
**created_at** | **String** | The date of the wallet transaction creation, represented in ISO 8601 datetime format and expressed in Coordinated Universal Time (UTC). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


