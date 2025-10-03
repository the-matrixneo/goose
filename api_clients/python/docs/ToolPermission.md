# ToolPermission


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**permission** | [**PermissionLevel**](PermissionLevel.md) |  | 
**tool_name** | **str** |  | 

## Example

```python
from goose_api.models.tool_permission import ToolPermission

# TODO update the JSON string below
json = "{}"
# create an instance of ToolPermission from a JSON string
tool_permission_instance = ToolPermission.from_json(json)
# print the JSON string representation of the object
print(ToolPermission.to_json())

# convert the object into a dict
tool_permission_dict = tool_permission_instance.to_dict()
# create an instance of ToolPermission from a dict
tool_permission_from_dict = ToolPermission.from_dict(tool_permission_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


