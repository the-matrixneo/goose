# MessageContentOneOf1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**annotations** | [**EmbeddedResourceAnnotations**](EmbeddedResourceAnnotations.md) |  | [optional] 
**data** | **str** |  | 
**mime_type** | **str** |  | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.message_content_one_of1 import MessageContentOneOf1

# TODO update the JSON string below
json = "{}"
# create an instance of MessageContentOneOf1 from a JSON string
message_content_one_of1_instance = MessageContentOneOf1.from_json(json)
# print the JSON string representation of the object
print(MessageContentOneOf1.to_json())

# convert the object into a dict
message_content_one_of1_dict = message_content_one_of1_instance.to_dict()
# create an instance of MessageContentOneOf1 from a dict
message_content_one_of1_from_dict = MessageContentOneOf1.from_dict(message_content_one_of1_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


