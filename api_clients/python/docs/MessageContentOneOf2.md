# MessageContentOneOf2


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**tool_call** | **object** |  | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.message_content_one_of2 import MessageContentOneOf2

# TODO update the JSON string below
json = "{}"
# create an instance of MessageContentOneOf2 from a JSON string
message_content_one_of2_instance = MessageContentOneOf2.from_json(json)
# print the JSON string representation of the object
print(MessageContentOneOf2.to_json())

# convert the object into a dict
message_content_one_of2_dict = message_content_one_of2_instance.to_dict()
# create an instance of MessageContentOneOf2 from a dict
message_content_one_of2_from_dict = MessageContentOneOf2.from_dict(message_content_one_of2_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


