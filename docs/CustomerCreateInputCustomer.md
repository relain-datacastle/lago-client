# CustomerCreateInputCustomer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_id** | **String** | The customer external unique identifier (provided by your own application) | 
**address_line1** | Option<**String**> | The first line of the billing address | [optional]
**address_line2** | Option<**String**> | The second line of the billing address | [optional]
**city** | Option<**String**> | The city of the customer's billing address | [optional]
**country** | Option<[**models::Country**](Country.md)> | Country code of the customer's billing address. Format must be ISO 3166 (alpha-2) | [optional]
**currency** | Option<[**models::Currency**](Currency.md)> | Currency of the customer. Format must be ISO 4217 | [optional]
**email** | Option<**String**> | The email of the customer | [optional]
**legal_name** | Option<**String**> | The legal company name of the customer | [optional]
**legal_number** | Option<**String**> | The legal company number of the customer | [optional]
**logo_url** | Option<**String**> | The logo URL of the customer | [optional]
**name** | Option<**String**> | The full name of the customer | [optional]
**firstname** | Option<**String**> | First name of the customer | [optional]
**lastname** | Option<**String**> | Last name of the customer | [optional]
**customer_type** | Option<**String**> | The type of the customer. It can have one of the following values: - `company`: the customer is a company. - `individual`: the customer is an individual. | [optional]
**phone** | Option<**String**> | The phone number of the customer | [optional]
**state** | Option<**String**> | The state of the customer's billing address | [optional]
**tax_codes** | Option<**Vec<String>**> | List of unique code used to identify the taxes. | [optional]
**tax_identification_number** | Option<**String**> | The tax identification number of the customer | [optional]
**timezone** | Option<[**models::Timezone**](Timezone.md)> | The customer's timezone, used for billing purposes in their local time. Overrides the organization's timezone | [optional]
**url** | Option<**String**> | The custom website URL of the customer | [optional]
**zipcode** | Option<**String**> | The zipcode of the customer's billing address | [optional]
**net_payment_term** | Option<**i32**> | The net payment term, expressed in days, specifies the duration within which a customer is expected to remit payment after the invoice is finalized. | [optional]
**finalize_zero_amount_invoice** | Option<**String**> | Specifies how invoices with a zero total amount should be handled: - `inherit`: (Default) Follows the organization-level configuration. - `finalize`: Invoices are issued and finalized even if the total amount is zero. - `skip`: Invoices with a total amount of zero are not finalized. | [optional]
**billing_configuration** | Option<[**models::CustomerBillingConfiguration**](CustomerBillingConfiguration.md)> |  | [optional]
**shipping_address** | Option<[**models::Address**](Address.md)> |  | [optional]
**integration_customers** | Option<[**Vec<models::CustomerCreateInputCustomerIntegrationCustomersInner>**](CustomerCreateInput_customer_integration_customers_inner.md)> |  | [optional]
**metadata** | Option<[**Vec<models::CustomerCreateInputCustomerMetadataInner>**](CustomerCreateInput_customer_metadata_inner.md)> | Set of key-value pairs that you can attach to a customer. This can be useful for storing additional information about the customer in a structured format | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


