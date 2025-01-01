# OrganizationUpdateInputOrganization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_url** | Option<**String**> | The URL of your newest updated webhook endpoint. This URL allows your organization to receive important messages, notifications, or data from the Lago system. By configuring your webhook endpoint to this URL, you can ensure that your organization stays informed and receives relevant information in a timely manner. | [optional]
**country** | Option<[**models::Country**](Country.md)> | The country of your organization. | [optional]
**default_currency** | Option<[**models::Currency**](Currency.md)> | The default currency of an organization. | [optional]
**address_line1** | Option<**String**> | The first line of your organization's billing address. | [optional]
**address_line2** | Option<**String**> | The second line of your organization's billing address. | [optional]
**state** | Option<**String**> | The state of your organization's billing address. | [optional]
**zipcode** | Option<**String**> | The zipcode of your organization's billing address. | [optional]
**email** | Option<**String**> | The email address of your organization used to bill your customers. | [optional]
**city** | Option<**String**> | The city of your organization's billing address. | [optional]
**legal_name** | Option<**String**> | The legal name of your organization. | [optional]
**legal_number** | Option<**String**> | The legal number of your organization. | [optional]
**document_numbering** | Option<**String**> | This parameter configures the method of incrementing invoice numbers for your customers.  - `per_customer`: Invoice numbers are incremented individually for each customer. This means every customer will have their own unique sequence of invoice numbers, separate from other customers. It ensures that each customer's invoice numbers follow a distinct and isolated numbering pattern. - `per_organization`: Invoice number incrementation is made across your entire organization. Rather than individual sequences for each customer, all invoices within the organization follow a single, unified numbering system. This creates a continuous and organization-wide sequence for all invoice numbers. Invoices are incremented per month (dynamic value used is YYYYMM), and invoice numbers are reset at the end of each month.  The default value for `document_numbering` is set to `per_customer`, meaning that, unless changed, invoice numbers will increment uniquely for each customer. | [optional]
**document_number_prefix** | Option<**String**> | Sets the prefix for invoices and credit notes. Default is the first three letters of your organization name plus the last four digits of your organization ID. Customizable within 1-10 characters, and automatically capitalized by Lago. | [optional]
**net_payment_term** | Option<**i32**> | The net payment term, expressed in days, specifies the duration within which a customer is expected to remit payment after the invoice is finalized. | [optional]
**tax_identification_number** | Option<**String**> | The tax identification number of your organization. | [optional]
**timezone** | Option<[**models::Timezone**](Timezone.md)> | Your organization's timezone, used for billing purposes in your own local time. Can be overwritten by the customer's timezone. | [optional]
**email_settings** | Option<**Vec<String>**> | Represents the email settings of the organization. It allows you to define which documents are sent by email. The field value determines the types of documents that trigger email notifications. Possible values for are `invoice.finalized` and `credit_note.created`. By configuring this field, you can specify whether invoices, credit notes, or both should be sent to recipients via email. | [optional]
**billing_configuration** | Option<[**models::OrganizationBillingConfiguration**](OrganizationBillingConfiguration.md)> |  | [optional]
**finalize_zero_amount_invoice** | Option<**bool**> | Determines whether invoices with a zero total amount should be finalized. If set to true, zero amount invoices will be finalized. If set to false, zero amount invoices will not be finalized. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


