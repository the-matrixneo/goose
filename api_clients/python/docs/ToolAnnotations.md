# ToolAnnotations


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destructive_hint** | **bool** |  | [optional] 
**idempotent_hint** | **bool** |  | [optional] 
**open_world_hint** | **bool** |  | [optional] 
**read_only_hint** | **bool** |  | [optional] 
**title** | **str** |  | [optional] 

## Example

```python
from goose_api.models.tool_annotations import ToolAnnotations

# TODO update the JSON string below
json = "{}"
# create an instance of ToolAnnotations from a JSON string
tool_annotations_instance = ToolAnnotations.from_json(json)
# print the JSON string representation of the object
print(ToolAnnotations.to_json())

# convert the object into a dict
tool_annotations_dict = tool_annotations_instance.to_dict()
# create an instance of ToolAnnotations from a dict
tool_annotations_from_dict = ToolAnnotations.from_dict(tool_annotations_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


