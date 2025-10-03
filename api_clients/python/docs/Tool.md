# Tool


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**annotations** | [**ToolAnnotations**](ToolAnnotations.md) |  | [optional] 
**description** | **str** |  | [optional] 
**icons** | [**List[Icon]**](Icon.md) |  | [optional] 
**input_schema** | **Dict[str, object]** |  | 
**name** | **str** |  | 
**output_schema** | **Dict[str, object]** |  | [optional] 
**title** | **str** |  | [optional] 

## Example

```python
from goose_api.models.tool import Tool

# TODO update the JSON string below
json = "{}"
# create an instance of Tool from a JSON string
tool_instance = Tool.from_json(json)
# print the JSON string representation of the object
print(Tool.to_json())

# convert the object into a dict
tool_dict = tool_instance.to_dict()
# create an instance of Tool from a dict
tool_from_dict = Tool.from_dict(tool_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


