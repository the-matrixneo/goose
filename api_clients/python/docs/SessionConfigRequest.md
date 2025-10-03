# SessionConfigRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**response** | [**Response**](Response.md) |  | [optional] 
**session_id** | **str** |  | 

## Example

```python
from goose_api.models.session_config_request import SessionConfigRequest

# TODO update the JSON string below
json = "{}"
# create an instance of SessionConfigRequest from a JSON string
session_config_request_instance = SessionConfigRequest.from_json(json)
# print the JSON string representation of the object
print(SessionConfigRequest.to_json())

# convert the object into a dict
session_config_request_dict = session_config_request_instance.to_dict()
# create an instance of SessionConfigRequest from a dict
session_config_request_from_dict = SessionConfigRequest.from_dict(session_config_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


