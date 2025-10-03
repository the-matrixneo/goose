# MessageContentOneOf5


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**tool_call** | **object** |  | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.message_content_one_of5 import MessageContentOneOf5

# TODO update the JSON string below
json = "{}"
# create an instance of MessageContentOneOf5 from a JSON string
message_content_one_of5_instance = MessageContentOneOf5.from_json(json)
# print the JSON string representation of the object
print(MessageContentOneOf5.to_json())

# convert the object into a dict
message_content_one_of5_dict = message_content_one_of5_instance.to_dict()
# create an instance of MessageContentOneOf5 from a dict
message_content_one_of5_from_dict = MessageContentOneOf5.from_dict(message_content_one_of5_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


