# SuccessCheck

Execute a shell command and check its exit status

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**command** | **str** | The shell command to execute | 
**type** | **str** |  | 

## Example

```python
from goose_api.models.success_check import SuccessCheck

# TODO update the JSON string below
json = "{}"
# create an instance of SuccessCheck from a JSON string
success_check_instance = SuccessCheck.from_json(json)
# print the JSON string representation of the object
print(SuccessCheck.to_json())

# convert the object into a dict
success_check_dict = success_check_instance.to_dict()
# create an instance of SuccessCheck from a dict
success_check_from_dict = SuccessCheck.from_dict(success_check_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


