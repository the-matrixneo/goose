# EmbeddedResourceAnnotations


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience** | **List[str]** |  | [optional] 
**last_modified** | **datetime** |  | [optional] 
**priority** | **float** |  | [optional] 

## Example

```python
from goose_api.models.embedded_resource_annotations import EmbeddedResourceAnnotations

# TODO update the JSON string below
json = "{}"
# create an instance of EmbeddedResourceAnnotations from a JSON string
embedded_resource_annotations_instance = EmbeddedResourceAnnotations.from_json(json)
# print the JSON string representation of the object
print(EmbeddedResourceAnnotations.to_json())

# convert the object into a dict
embedded_resource_annotations_dict = embedded_resource_annotations_instance.to_dict()
# create an instance of EmbeddedResourceAnnotations from a dict
embedded_resource_annotations_from_dict = EmbeddedResourceAnnotations.from_dict(embedded_resource_annotations_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


