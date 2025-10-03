# ExtensionConfigOneOf3

Streamable HTTP client with a URI endpoint using MCP Streamable HTTP specification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_tools** | **List[str]** |  | [optional] 
**bundled** | **bool** | Whether this extension is bundled with goose | [optional] 
**description** | **str** |  | [optional] 
**env_keys** | **List[str]** |  | [optional] 
**envs** | **Dict[str, str]** |  | [optional] 
**headers** | **Dict[str, str]** |  | [optional] 
**name** | **str** | The name used to identify this extension | 
**timeout** | **int** |  | [optional] 
**type** | **str** |  | 
**uri** | **str** |  | 

## Example

```python
from goose_api.models.extension_config_one_of3 import ExtensionConfigOneOf3

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionConfigOneOf3 from a JSON string
extension_config_one_of3_instance = ExtensionConfigOneOf3.from_json(json)
# print the JSON string representation of the object
print(ExtensionConfigOneOf3.to_json())

# convert the object into a dict
extension_config_one_of3_dict = extension_config_one_of3_instance.to_dict()
# create an instance of ExtensionConfigOneOf3 from a dict
extension_config_one_of3_from_dict = ExtensionConfigOneOf3.from_dict(extension_config_one_of3_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


