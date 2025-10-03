# MessageContentOneOf


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**annotations** | [**EmbeddedResourceAnnotations**](EmbeddedResourceAnnotations.md) |  | [optional] 
**text** | **str** |  | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.message_content_one_of import MessageContentOneOf

# TODO update the JSON string below
json = "{}"
# create an instance of MessageContentOneOf from a JSON string
message_content_one_of_instance = MessageContentOneOf.from_json(json)
# print the JSON string representation of the object
print(MessageContentOneOf.to_json())

# convert the object into a dict
message_content_one_of_dict = message_content_one_of_instance.to_dict()
# create an instance of MessageContentOneOf from a dict
message_content_one_of_from_dict = MessageContentOneOf.from_dict(message_content_one_of_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


