# MessageContentOneOf3


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**tool_result** | **object** |  | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.message_content_one_of3 import MessageContentOneOf3

# TODO update the JSON string below
json = "{}"
# create an instance of MessageContentOneOf3 from a JSON string
message_content_one_of3_instance = MessageContentOneOf3.from_json(json)
# print the JSON string representation of the object
print(MessageContentOneOf3.to_json())

# convert the object into a dict
message_content_one_of3_dict = message_content_one_of3_instance.to_dict()
# create an instance of MessageContentOneOf3 from a dict
message_content_one_of3_from_dict = MessageContentOneOf3.from_dict(message_content_one_of3_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


