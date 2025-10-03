# MessageContentOneOf4


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arguments** | **Dict[str, object]** |  | 
**id** | **str** |  | 
**prompt** | **str** |  | [optional] 
**tool_name** | **str** |  | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.message_content_one_of4 import MessageContentOneOf4

# TODO update the JSON string below
json = "{}"
# create an instance of MessageContentOneOf4 from a JSON string
message_content_one_of4_instance = MessageContentOneOf4.from_json(json)
# print the JSON string representation of the object
print(MessageContentOneOf4.to_json())

# convert the object into a dict
message_content_one_of4_dict = message_content_one_of4_instance.to_dict()
# create an instance of MessageContentOneOf4 from a dict
message_content_one_of4_from_dict = MessageContentOneOf4.from_dict(message_content_one_of4_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


