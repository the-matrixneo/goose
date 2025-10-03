# ExtendPromptRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extension** | **str** |  | 
**session_id** | **str** |  | 

## Example

```python
from goose_api.models.extend_prompt_request import ExtendPromptRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ExtendPromptRequest from a JSON string
extend_prompt_request_instance = ExtendPromptRequest.from_json(json)
# print the JSON string representation of the object
print(ExtendPromptRequest.to_json())

# convert the object into a dict
extend_prompt_request_dict = extend_prompt_request_instance.to_dict()
# create an instance of ExtendPromptRequest from a dict
extend_prompt_request_from_dict = ExtendPromptRequest.from_dict(extend_prompt_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


