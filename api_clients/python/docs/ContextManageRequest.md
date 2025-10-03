# ContextManageRequest

Request payload for context management operations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**manage_action** | **str** | Operation to perform: \&quot;truncation\&quot; or \&quot;summarize\&quot; | 
**messages** | [**List[Message]**](Message.md) | Collection of messages to be managed | 
**session_id** | **str** | Optional session ID for session-specific agent | 

## Example

```python
from goose_api.models.context_manage_request import ContextManageRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ContextManageRequest from a JSON string
context_manage_request_instance = ContextManageRequest.from_json(json)
# print the JSON string representation of the object
print(ContextManageRequest.to_json())

# convert the object into a dict
context_manage_request_dict = context_manage_request_instance.to_dict()
# create an instance of ContextManageRequest from a dict
context_manage_request_from_dict = ContextManageRequest.from_dict(context_manage_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


