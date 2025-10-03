# UpsertConfigQuery


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_secret** | **bool** |  | 
**key** | **str** |  | 
**value** | **object** |  | 

## Example

```python
from goose_api.models.upsert_config_query import UpsertConfigQuery

# TODO update the JSON string below
json = "{}"
# create an instance of UpsertConfigQuery from a JSON string
upsert_config_query_instance = UpsertConfigQuery.from_json(json)
# print the JSON string representation of the object
print(UpsertConfigQuery.to_json())

# convert the object into a dict
upsert_config_query_dict = upsert_config_query_instance.to_dict()
# create an instance of UpsertConfigQuery from a dict
upsert_config_query_from_dict = UpsertConfigQuery.from_dict(upsert_config_query_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


