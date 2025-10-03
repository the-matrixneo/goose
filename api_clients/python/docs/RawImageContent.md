# RawImageContent


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**data** | **str** |  | 
**mime_type** | **str** |  | 

## Example

```python
from goose_api.models.raw_image_content import RawImageContent

# TODO update the JSON string below
json = "{}"
# create an instance of RawImageContent from a JSON string
raw_image_content_instance = RawImageContent.from_json(json)
# print the JSON string representation of the object
print(RawImageContent.to_json())

# convert the object into a dict
raw_image_content_dict = raw_image_content_instance.to_dict()
# create an instance of RawImageContent from a dict
raw_image_content_from_dict = RawImageContent.from_dict(raw_image_content_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


