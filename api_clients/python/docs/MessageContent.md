# MessageContent

Content passed inside a message, which can be both simple content and tool content

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**annotations** | [**EmbeddedResourceAnnotations**](EmbeddedResourceAnnotations.md) |  | [optional] 
**text** | **str** |  | 
**type** | **str** |  | 
**data** | **str** |  | 
**mime_type** | **str** |  | 
**id** | **str** |  | 
**tool_call** | **object** |  | 
**tool_result** | **object** |  | 
**arguments** | **Dict[str, object]** |  | 
**prompt** | **str** |  | [optional] 
**tool_name** | **str** |  | 
**signature** | **str** |  | 
**thinking** | **str** |  | 
**msg** | **str** |  | 

## Example

```python
from goose_api.models.message_content import MessageContent

# TODO update the JSON string below
json = "{}"
# create an instance of MessageContent from a JSON string
message_content_instance = MessageContent.from_json(json)
# print the JSON string representation of the object
print(MessageContent.to_json())

# convert the object into a dict
message_content_dict = message_content_instance.to_dict()
# create an instance of MessageContent from a dict
message_content_from_dict = MessageContent.from_dict(message_content_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


