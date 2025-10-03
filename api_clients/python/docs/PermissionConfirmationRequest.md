# PermissionConfirmationRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **str** |  | 
**id** | **str** |  | 
**principal_type** | [**PrincipalType**](PrincipalType.md) |  | [optional] 
**session_id** | **str** |  | 

## Example

```python
from goose_api.models.permission_confirmation_request import PermissionConfirmationRequest

# TODO update the JSON string below
json = "{}"
# create an instance of PermissionConfirmationRequest from a JSON string
permission_confirmation_request_instance = PermissionConfirmationRequest.from_json(json)
# print the JSON string representation of the object
print(PermissionConfirmationRequest.to_json())

# convert the object into a dict
permission_confirmation_request_dict = permission_confirmation_request_instance.to_dict()
# create an instance of PermissionConfirmationRequest from a dict
permission_confirmation_request_from_dict = PermissionConfirmationRequest.from_dict(permission_confirmation_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


