# ExtensionConfigOneOf

Server-sent events client with a URI endpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_tools** | **List[str]** |  | [optional] 
**bundled** | **bool** | Whether this extension is bundled with goose | [optional] 
**description** | **str** |  | [optional] 
**env_keys** | **List[str]** |  | [optional] 
**envs** | **Dict[str, str]** |  | [optional] 
**name** | **str** | The name used to identify this extension | 
**timeout** | **int** |  | [optional] 
**type** | **str** |  | 
**uri** | **str** |  | 

## Example

```python
from goose_api.models.extension_config_one_of import ExtensionConfigOneOf

# TODO update the JSON string below
json = "{}"
# create an instance of ExtensionConfigOneOf from a JSON string
extension_config_one_of_instance = ExtensionConfigOneOf.from_json(json)
# print the JSON string representation of the object
print(ExtensionConfigOneOf.to_json())

# convert the object into a dict
extension_config_one_of_dict = extension_config_one_of_instance.to_dict()
# create an instance of ExtensionConfigOneOf from a dict
extension_config_one_of_from_dict = ExtensionConfigOneOf.from_dict(extension_config_one_of_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


