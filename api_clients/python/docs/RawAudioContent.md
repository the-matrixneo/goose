# RawAudioContent


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | **str** |  | 
**mime_type** | **str** |  | 

## Example

```python
from goose_api.models.raw_audio_content import RawAudioContent

# TODO update the JSON string below
json = "{}"
# create an instance of RawAudioContent from a JSON string
raw_audio_content_instance = RawAudioContent.from_json(json)
# print the JSON string representation of the object
print(RawAudioContent.to_json())

# convert the object into a dict
raw_audio_content_dict = raw_audio_content_instance.to_dict()
# create an instance of RawAudioContent from a dict
raw_audio_content_from_dict = RawAudioContent.from_dict(raw_audio_content_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


