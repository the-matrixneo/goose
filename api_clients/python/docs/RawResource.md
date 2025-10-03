# RawResource


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **str** |  | [optional] 
**icons** | [**List[Icon]**](Icon.md) |  | [optional] 
**mime_type** | **str** |  | [optional] 
**name** | **str** |  | 
**size** | **int** |  | [optional] 
**title** | **str** |  | [optional] 
**uri** | **str** |  | 

## Example

```python
from goose_api.models.raw_resource import RawResource

# TODO update the JSON string below
json = "{}"
# create an instance of RawResource from a JSON string
raw_resource_instance = RawResource.from_json(json)
# print the JSON string representation of the object
print(RawResource.to_json())

# convert the object into a dict
raw_resource_dict = raw_resource_instance.to_dict()
# create an instance of RawResource from a dict
raw_resource_from_dict = RawResource.from_dict(raw_resource_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


