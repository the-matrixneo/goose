# ExtensionConfigOneOf1

Standard I/O client with command and arguments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args** | **List[str]** |  | 
**available_tools** | **List[str]** |  | [optional] 
**bundled** | **bool** | Whether this extension is bundled with goose | [optional] 
**cmd** | **str** |  | 
**description** | **str** |  | [optional] 
**env_keys** | **List[str]** |  | [optional] 
**envs** | **Dict[str, str]** |  | [optional] 
**name** | **str** | The name used to identify this extension | 
**timeout** | **int** |  | [optional] 
**type** | **str** |  | 

## Example

```python
from goose_api.models.extension_config_one_of1 import ExtensionConfigOneOf1

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionConfigOneOf1 from a JSON string
extension_config_one_of1_instance = ExtensionConfigOneOf1.from_json(json)
# print the JSON string representation of the object
print(ExtensionConfigOneOf1.to_json())

# convert the object into a dict
extension_config_one_of1_dict = extension_config_one_of1_instance.to_dict()
# create an instance of ExtensionConfigOneOf1 from a dict
extension_config_one_of1_from_dict = ExtensionConfigOneOf1.from_dict(extension_config_one_of1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


