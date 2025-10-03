# ExtensionConfigOneOf2

Built-in extension that is part of the goose binary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_tools** | **List[str]** |  | [optional] 
**bundled** | **bool** | Whether this extension is bundled with goose | [optional] 
**description** | **str** |  | [optional] 
**display_name** | **str** |  | [optional] 
**name** | **str** | The name used to identify this extension | 
**timeout** | **int** |  | [optional] 
**type** | **str** |  | 

## Example

```python
from goose_api.models.extension_config_one_of2 import ExtensionConfigOneOf2

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionConfigOneOf2 from a JSON string
extension_config_one_of2_instance = ExtensionConfigOneOf2.from_json(json)
# print the JSON string representation of the object
print(ExtensionConfigOneOf2.to_json())

# convert the object into a dict
extension_config_one_of2_dict = extension_config_one_of2_instance.to_dict()
# create an instance of ExtensionConfigOneOf2 from a dict
extension_config_one_of2_from_dict = ExtensionConfigOneOf2.from_dict(extension_config_one_of2_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


