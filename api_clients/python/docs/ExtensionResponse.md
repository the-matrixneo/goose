# ExtensionResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extensions** | [**List[ExtensionEntry]**](ExtensionEntry.md) |  | 

## Example

```python
from goose_api.models.extension_response import ExtensionResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionResponse from a JSON string
extension_response_instance = ExtensionResponse.from_json(json)
# print the JSON string representation of the object
print(ExtensionResponse.to_json())

# convert the object into a dict
extension_response_dict = extension_response_instance.to_dict()
# create an instance of ExtensionResponse from a dict
extension_response_from_dict = ExtensionResponse.from_dict(extension_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


