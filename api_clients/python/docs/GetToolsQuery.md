# GetToolsQuery


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extension_name** | **str** |  | [optional] 
**session_id** | **str** |  | 

## Example

```python
from goose_api.models.get_tools_query import GetToolsQuery

# TODO update the JSON string below
json = "{}"
# create an instance of GetToolsQuery from a JSON string
get_tools_query_instance = GetToolsQuery.from_json(json)
# print the JSON string representation of the object
print(GetToolsQuery.to_json())

# convert the object into a dict
get_tools_query_dict = get_tools_query_instance.to_dict()
# create an instance of GetToolsQuery from a dict
get_tools_query_from_dict = GetToolsQuery.from_dict(get_tools_query_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


