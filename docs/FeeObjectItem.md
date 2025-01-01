# FeeObjectItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The fee type. Possible values are `add-on`, `charge`, `credit` or `subscription`. | 
**code** | **String** | The code of the fee item. It can be the code of the `add-o`n, the code of the `charge`, the code of the `credit` or the code of the `subscription`. | 
**name** | **String** | The name of the fee item. It can be the name of the `add-on`, the name of the `charge`, the name of the `credit` or the name of the `subscription`. | 
**invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the name of the actual charge will be used as the default display name. | [optional]
**filter_invoice_display_name** | Option<**String**> | Specifies the name that will be displayed on an invoice. If no value is set for this field, the actual charge filter values will be used as the default display name. | [optional]
**filters** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | Key value list of event properties | [optional]
**lago_item_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the fee item, created by Lago. It can be the identifier of the `add-on`, the identifier of the `charge`, the identifier of the `credit` or the identifier of the `subscription`. | 
**item_type** | **String** | The type of the fee item. Possible values are `AddOn`, `BillableMetric`, `WalletTransaction` or `Subscription`. | 
**grouped_by** | Option<**std::collections::HashMap<String, String>**> | Key value list of event properties aggregated by the charge model | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


