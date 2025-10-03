# FrontendToolRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**tool_call** | **object** |  | 

## Example

```python
from goose_api.models.frontend_tool_request import FrontendToolRequest

# TODO update the JSON string below
json = "{}"
# create an instance of FrontendToolRequest from a JSON string
frontend_tool_request_instance = FrontendToolRequest.from_json(json)
# print the JSON string representation of the object
print(FrontendToolRequest.to_json())

# convert the object into a dict
frontend_tool_request_dict = frontend_tool_request_instance.to_dict()
# create an instance of FrontendToolRequest from a dict
frontend_tool_request_from_dict = FrontendToolRequest.from_dict(frontend_tool_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


