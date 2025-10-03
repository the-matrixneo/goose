# EmbeddedResource


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**annotations** | [**EmbeddedResourceAnnotations**](EmbeddedResourceAnnotations.md) |  | [optional] 
**resource** | [**ResourceContents**](ResourceContents.md) |  | 

## Example

```python
from goose_api.models.embedded_resource import EmbeddedResource

# TODO update the JSON string below
json = "{}"
# create an instance of EmbeddedResource from a JSON string
embedded_resource_instance = EmbeddedResource.from_json(json)
# print the JSON string representation of the object
print(EmbeddedResource.to_json())

# convert the object into a dict
embedded_resource_dict = embedded_resource_instance.to_dict()
# create an instance of EmbeddedResource from a dict
embedded_resource_from_dict = EmbeddedResource.from_dict(embedded_resource_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


