# ExtensionConfigOneOf5

Inline Python code that will be executed using uvx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_tools** | **List[str]** |  | [optional] 
**code** | **str** | The Python code to execute | 
**dependencies** | **List[str]** | Python package dependencies required by this extension | [optional] 
**description** | **str** | Description of what the extension does | [optional] 
**name** | **str** | The name used to identify this extension | 
**timeout** | **int** | Timeout in seconds | [optional] 
**type** | **str** |  | 

## Example

```python
from goose_api.models.extension_config_one_of5 import ExtensionConfigOneOf5

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionConfigOneOf5 from a JSON string
extension_config_one_of5_instance = ExtensionConfigOneOf5.from_json(json)
# print the JSON string representation of the object
print(ExtensionConfigOneOf5.to_json())

# convert the object into a dict
extension_config_one_of5_dict = extension_config_one_of5_instance.to_dict()
# create an instance of ExtensionConfigOneOf5 from a dict
extension_config_one_of5_from_dict = ExtensionConfigOneOf5.from_dict(extension_config_one_of5_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


