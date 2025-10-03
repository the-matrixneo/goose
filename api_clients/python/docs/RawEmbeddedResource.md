# RawEmbeddedResource


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**resource** | [**ResourceContents**](ResourceContents.md) |  | 

## Example

```python
from goose_api.models.raw_embedded_resource import RawEmbeddedResource

# TODO update the JSON string below
json = "{}"
# create an instance of RawEmbeddedResource from a JSON string
raw_embedded_resource_instance = RawEmbeddedResource.from_json(json)
# print the JSON string representation of the object
print(RawEmbeddedResource.to_json())

# convert the object into a dict
raw_embedded_resource_dict = raw_embedded_resource_instance.to_dict()
# create an instance of RawEmbeddedResource from a dict
raw_embedded_resource_from_dict = RawEmbeddedResource.from_dict(raw_embedded_resource_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


