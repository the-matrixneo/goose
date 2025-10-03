# ExtensionQuery


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | [**ExtensionConfig**](ExtensionConfig.md) |  | 
**enabled** | **bool** |  | 
**name** | **str** |  | 

## Example

```python
from goose_api.models.extension_query import ExtensionQuery

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionQuery from a JSON string
extension_query_instance = ExtensionQuery.from_json(json)
# print the JSON string representation of the object
print(ExtensionQuery.to_json())

# convert the object into a dict
extension_query_dict = extension_query_instance.to_dict()
# create an instance of ExtensionQuery from a dict
extension_query_from_dict = ExtensionQuery.from_dict(extension_query_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


