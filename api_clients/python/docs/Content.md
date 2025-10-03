# Content


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**text** | **str** |  | 
**data** | **str** |  | 
**mime_type** | **str** |  | 
**resource** | [**ResourceContents**](ResourceContents.md) |  | 
**description** | **str** |  | [optional] 
**icons** | [**List[Icon]**](Icon.md) |  | [optional] 
**name** | **str** |  | 
**size** | **int** |  | [optional] 
**title** | **str** |  | [optional] 
**uri** | **str** |  | 

## Example

```python
from goose_api.models.content import Content

# TODO update the JSON string below
json = "{}"
# create an instance of Content from a JSON string
content_instance = Content.from_json(json)
# print the JSON string representation of the object
print(Content.to_json())

# convert the object into a dict
content_dict = content_instance.to_dict()
# create an instance of Content from a dict
content_from_dict = Content.from_dict(content_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


