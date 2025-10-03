# UpdateProviderRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | **str** |  | [optional] 
**provider** | **str** |  | 
**session_id** | **str** |  | 

## Example

```python
from goose_api.models.update_provider_request import UpdateProviderRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateProviderRequest from a JSON string
update_provider_request_instance = UpdateProviderRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateProviderRequest.to_json())

# convert the object into a dict
update_provider_request_dict = update_provider_request_instance.to_dict()
# create an instance of UpdateProviderRequest from a dict
update_provider_request_from_dict = UpdateProviderRequest.from_dict(update_provider_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


