# ToolConfirmationRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arguments** | **Dict[str, object]** |  | 
**id** | **str** |  | 
**prompt** | **str** |  | [optional] 
**tool_name** | **str** |  | 

## Example

```python
from goose_api.models.tool_confirmation_request import ToolConfirmationRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ToolConfirmationRequest from a JSON string
tool_confirmation_request_instance = ToolConfirmationRequest.from_json(json)
# print the JSON string representation of the object
print(ToolConfirmationRequest.to_json())

# convert the object into a dict
tool_confirmation_request_dict = tool_confirmation_request_instance.to_dict()
# create an instance of ToolConfirmationRequest from a dict
tool_confirmation_request_from_dict = ToolConfirmationRequest.from_dict(tool_confirmation_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


