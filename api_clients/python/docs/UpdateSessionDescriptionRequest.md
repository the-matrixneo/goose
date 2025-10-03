# UpdateSessionDescriptionRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **str** | Updated description (name) for the session (max 200 characters) | 

## Example

```python
from goose_api.models.update_session_description_request import UpdateSessionDescriptionRequest

# TODO update the JSON string below
json = "{}"
# create an instance of UpdateSessionDescriptionRequest from a JSON string
update_session_description_request_instance = UpdateSessionDescriptionRequest.from_json(json)
# print the JSON string representation of the object
print(UpdateSessionDescriptionRequest.to_json())

# convert the object into a dict
update_session_description_request_dict = update_session_description_request_instance.to_dict()
# create an instance of UpdateSessionDescriptionRequest from a dict
update_session_description_request_from_dict = UpdateSessionDescriptionRequest.from_dict(update_session_description_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


