# CreditNoteEstimatedEstimatedCreditNote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_invoice_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the invoice that the credit note belongs to | 
**invoice_number** | **String** | The invoice unique number, related to the credit note. | 
**currency** | [**models::Currency**](Currency.md) | The currency of the credit note. | 
**taxes_amount_cents** | **i32** | The tax amount of the credit note, expressed in cents. | 
**taxes_rate** | **f64** | The tax rate associated with this specific credit note. | 
**sub_total_excluding_taxes_amount_cents** | **i32** | The subtotal of the credit note excluding any applicable taxes, expressed in cents. | 
**max_creditable_amount_cents** | **i32** | The credited amount of the credit note, expressed in cents. | 
**max_refundable_amount_cents** | **i32** | The refunded amount of the credit note, expressed in cents. | 
**coupons_adjustment_amount_cents** | **i32** | The pro-rated amount of the coupons applied to the source invoice. | 
**items** | [**Vec<models::CreditNoteEstimatedEstimatedCreditNoteItemsInner>**](CreditNoteEstimated_estimated_credit_note_items_inner.md) | Array of credit note's items. | 
**applied_taxes** | Option<[**Vec<models::CreditNoteEstimatedEstimatedCreditNoteAppliedTaxesInner>**](CreditNoteEstimated_estimated_credit_note_applied_taxes_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


