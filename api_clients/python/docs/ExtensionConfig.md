# ExtensionConfig

Represents the different types of MCP extensions that can be added to the manager

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_tools** | **List[str]** |  | [optional] 
**bundled** | **bool** | Whether this extension is bundled with goose | [optional] 
**description** | **str** | Description of what the extension does | [optional] 
**env_keys** | **List[str]** |  | [optional] 
**envs** | **Dict[str, str]** |  | [optional] 
**name** | **str** | The name used to identify this extension | 
**timeout** | **int** | Timeout in seconds | [optional] 
**type** | **str** |  | 
**uri** | **str** |  | 
**args** | **List[str]** |  | 
**cmd** | **str** |  | 
**display_name** | **str** |  | [optional] 
**headers** | **Dict[str, str]** |  | [optional] 
**instructions** | **str** | Instructions for how to use these tools | [optional] 
**tools** | [**List[Tool]**](Tool.md) | The tools provided by the frontend | 
**code** | **str** | The Python code to execute | 
**dependencies** | **List[str]** | Python package dependencies required by this extension | [optional] 

## Example

```python
from goose_api.models.extension_config import ExtensionConfig

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionConfig from a JSON string
extension_config_instance = ExtensionConfig.from_json(json)
# print the JSON string representation of the object
print(ExtensionConfig.to_json())

# convert the object into a dict
extension_config_dict = extension_config_instance.to_dict()
# create an instance of ExtensionConfig from a dict
extension_config_from_dict = ExtensionConfig.from_dict(extension_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


