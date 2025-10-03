# ResourceContentsAnyOf


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**mime_type** | **str** |  | [optional] 
**text** | **str** |  | 
**uri** | **str** |  | 

## Example

```python
from goose_api.models.resource_contents_any_of import ResourceContentsAnyOf

# TODO update the JSON string below
json = "{}"
# create an instance of ResourceContentsAnyOf from a JSON string
resource_contents_any_of_instance = ResourceContentsAnyOf.from_json(json)
# print the JSON string representation of the object
print(ResourceContentsAnyOf.to_json())

# convert the object into a dict
resource_contents_any_of_dict = resource_contents_any_of_instance.to_dict()
# create an instance of ResourceContentsAnyOf from a dict
resource_contents_any_of_from_dict = ResourceContentsAnyOf.from_dict(resource_contents_any_of_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


