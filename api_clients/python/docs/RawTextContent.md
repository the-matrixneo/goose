# RawTextContent


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**meta** | **Dict[str, object]** |  | [optional] 
**text** | **str** |  | 

## Example

```python
from goose_api.models.raw_text_content import RawTextContent

# TODO update the JSON string below
json = "{}"
# create an instance of RawTextContent from a JSON string
raw_text_content_instance = RawTextContent.from_json(json)
# print the JSON string representation of the object
print(RawTextContent.to_json())

# convert the object into a dict
raw_text_content_dict = raw_text_content_instance.to_dict()
# create an instance of RawTextContent from a dict
raw_text_content_from_dict = RawTextContent.from_dict(raw_text_content_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


