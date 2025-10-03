# ContextManageResponse

Response from context management operations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**messages** | [**List[Message]**](Message.md) | Processed messages after the operation | 
**token_counts** | **List[int]** | Token counts for each processed message | 

## Example

```python
from goose_api.models.context_manage_response import ContextManageResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ContextManageResponse from a JSON string
context_manage_response_instance = ContextManageResponse.from_json(json)
# print the JSON string representation of the object
print(ContextManageResponse.to_json())

# convert the object into a dict
context_manage_response_dict = context_manage_response_instance.to_dict()
# create an instance of ContextManageResponse from a dict
context_manage_response_from_dict = ContextManageResponse.from_dict(context_manage_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


