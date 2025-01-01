# CreditNoteObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) | The credit note unique identifier, created by Lago. | 
**sequential_id** | **i32** | The sequential identifier of the credit note, specifically scoped on the associated invoice. It provides a unique numerical identifier for the credit note within the context of the invoice. | 
**number** | **String** | The credit note unique number. | 
**lago_invoice_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier assigned to the invoice that the credit note belongs to | 
**invoice_number** | **String** | The invoice unique number, related to the credit note. | 
**issuing_date** | [**String**](string.md) | The date of creation of the credit note. It follows the ISO 8601 date format and provides the specific date when the credit note was created. | 
**credit_status** | Option<**String**> | The status of the credit portion of the credit note. It indicates the current state or condition of the credit amount associated with the credit note. The possible values for this field are:  - `available`: this status indicates that an amount remains available for future usage. The credit can be applied towards future transactions or invoices. - `consumed`: this status indicates that the credit amount has been fully consumed. The remaining amount is 0, indicating that the credit has been utilized in its entirety. - `voided`: this status indicates that the remaining amount of the credit cannot be used any further. The credit has been voided and is no longer available for application or redemption. | [optional]
**refund_status** | Option<**String**> | The status of the refund portion of the credit note. It indicates the current state or condition of the refund associated with the credit note. The possible values for this field are:  - `pending`: this status indicates that the refund is pending execution. The refund request has been initiated but has not been processed or completed yet. - `succeeded`: this status indicates that the refund has been successfully executed. The refund amount has been processed and returned to the customer or the designated recipient. - `failed`: this status indicates that the refund failed to execute. The refund request encountered an error or unsuccessful processing, and the refund amount could not be returned. | [optional]
**reason** | **String** | The reason of the credit note creation. Possible values are `duplicated_charge`, `product_unsatisfactory`, `order_change`, `order_cancellation`, `fraudulent_charge` or `other`. | 
**description** | Option<**String**> | The description of the credit note. | [optional]
**currency** | [**models::Currency**](Currency.md) | The currency of the credit note. | 
**total_amount_cents** | **i32** | The total amount of the credit note, expressed in cents. | 
**taxes_amount_cents** | **i32** | The tax amount of the credit note, expressed in cents. | 
**taxes_rate** | **f64** | The tax rate associated with this specific credit note. | 
**sub_total_excluding_taxes_amount_cents** | **i32** | The subtotal of the credit note excluding any applicable taxes, expressed in cents. | 
**balance_amount_cents** | **i32** | The remaining credit note amount, expressed in cents. | 
**credit_amount_cents** | **i32** | The credited amount of the credit note, expressed in cents. | 
**refund_amount_cents** | **i32** | The refunded amount of the credit note, expressed in cents. | 
**coupons_adjustment_amount_cents** | **i32** | The pro-rated amount of the coupons applied to the source invoice. | 
**created_at** | **String** | The date when the credit note was created. It is expressed in Coordinated Universal Time (UTC). | 
**updated_at** | **String** | The date when the credit note was last updated. It is expressed in Coordinated Universal Time (UTC). | 
**file_url** | Option<**String**> | The PDF file of the credit note. | [optional]
**items** | Option<[**Vec<models::CreditNoteItemObject>**](CreditNoteItemObject.md)> | Array of credit note's items. | [optional]
**applied_taxes** | Option<[**Vec<models::CreditNoteAppliedTaxObject>**](CreditNoteAppliedTaxObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


