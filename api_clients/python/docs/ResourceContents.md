# ResourceContents


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**mime_type** | **str** |  | [optional] 
**text** | **str** |  | 
**uri** | **str** |  | 
**blob** | **str** |  | 

## Example

```python
from goose_api.models.resource_contents import ResourceContents

# TODO update the JSON string below
json = "{}"
# create an instance of ResourceContents from a JSON string
resource_contents_instance = ResourceContents.from_json(json)
# print the JSON string representation of the object
print(ResourceContents.to_json())

# convert the object into a dict
resource_contents_dict = resource_contents_instance.to_dict()
# create an instance of ResourceContents from a dict
resource_contents_from_dict = ResourceContents.from_dict(resource_contents_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


