# CustomerCreateInputCustomerIntegrationCustomersInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique identifier for the integration customer object in the Lago application. | [optional]
**integration_type** | Option<**String**> | The integration type used for accounting and tax syncs. Accepted values: `netsuite, anrok, xero`. | [optional]
**integration_code** | Option<**String**> | Unique code used to identify an integration connection. | [optional]
**external_customer_id** | Option<**String**> | The customer ID within the integration's system. If this field is not provided, Lago has the option to create a new customer record within the integration's system on behalf of the customer. | [optional]
**sync_with_provider** | Option<**bool**> | Set this field to `true` if you want to create a customer record in the integration's system. This option is applicable only when the `external_customer_id` is null and the `sync_with_provider` field is set to `true`. By default, the value is set to `false` | [optional]
**subsidiary_id** | Option<**String**> | This optional field is needed only when working with `netsuite` connection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


