# CreditNoteCreateInputCreditNote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_id** | [**uuid::Uuid**](uuid::Uuid.md) | The invoice unique identifier, created by Lago. | 
**reason** | Option<**String**> | The reason of the credit note creation. Possible values are `duplicated_charge`, `product_unsatisfactory`, `order_change`, `order_cancellation`, `fraudulent_charge` or `other`. | [optional]
**description** | Option<**String**> | The description of the credit note. | [optional]
**credit_amount_cents** | Option<**i32**> | The total amount to be credited on the customer balance. | [optional]
**refund_amount_cents** | Option<**i32**> | The total amount to be refunded to the customer. | [optional]
**items** | [**Vec<models::CreditNoteEstimateInputCreditNoteItemsInner>**](CreditNoteEstimateInput_credit_note_items_inner.md) | The list of credit note's items. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


