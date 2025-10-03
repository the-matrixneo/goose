# ThinkingContent


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signature** | **str** |  | 
**thinking** | **str** |  | 

## Example

```python
from goose_api.models.thinking_content import ThinkingContent

# TODO update the JSON string below
json = "{}"
# create an instance of ThinkingContent from a JSON string
thinking_content_instance = ThinkingContent.from_json(json)
# print the JSON string representation of the object
print(ThinkingContent.to_json())

# convert the object into a dict
thinking_content_dict = thinking_content_instance.to_dict()
# create an instance of ThinkingContent from a dict
thinking_content_from_dict = ThinkingContent.from_dict(thinking_content_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


