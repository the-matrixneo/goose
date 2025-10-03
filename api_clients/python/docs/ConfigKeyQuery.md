# ConfigKeyQuery


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_secret** | **bool** |  | 
**key** | **str** |  | 

## Example

```python
from goose_api.models.config_key_query import ConfigKeyQuery

# TODO update the JSON string below
json = "{}"
# create an instance of ConfigKeyQuery from a JSON string
config_key_query_instance = ConfigKeyQuery.from_json(json)
# print the JSON string representation of the object
print(ConfigKeyQuery.to_json())

# convert the object into a dict
config_key_query_dict = config_key_query_instance.to_dict()
# create an instance of ConfigKeyQuery from a dict
config_key_query_from_dict = ConfigKeyQuery.from_dict(config_key_query_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


