# ExtensionConfigOneOf4

Frontend-provided tools that will be called through the frontend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_tools** | **List[str]** |  | [optional] 
**bundled** | **bool** | Whether this extension is bundled with goose | [optional] 
**instructions** | **str** | Instructions for how to use these tools | [optional] 
**name** | **str** | The name used to identify this extension | 
**tools** | [**List[Tool]**](Tool.md) | The tools provided by the frontend | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.extension_config_one_of4 import ExtensionConfigOneOf4

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionConfigOneOf4 from a JSON string
extension_config_one_of4_instance = ExtensionConfigOneOf4.from_json(json)
# print the JSON string representation of the object
print(ExtensionConfigOneOf4.to_json())

# convert the object into a dict
extension_config_one_of4_dict = extension_config_one_of4_instance.to_dict()
# create an instance of ExtensionConfigOneOf4 from a dict
extension_config_one_of4_from_dict = ExtensionConfigOneOf4.from_dict(extension_config_one_of4_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


