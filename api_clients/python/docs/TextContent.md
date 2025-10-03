# TextContent


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**annotations** | [**EmbeddedResourceAnnotations**](EmbeddedResourceAnnotations.md) |  | [optional] 
**text** | **str** |  | 

## Example

```python
from goose_api.models.text_content import TextContent

# TODO update the JSON string below
json = "{}"
# create an instance of TextContent from a JSON string
text_content_instance = TextContent.from_json(json)
# print the JSON string representation of the object
print(TextContent.to_json())

# convert the object into a dict
text_content_dict = text_content_instance.to_dict()
# create an instance of TextContent from a dict
text_content_from_dict = TextContent.from_dict(text_content_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


