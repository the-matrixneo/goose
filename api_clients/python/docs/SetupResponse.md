# SetupResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **str** |  | 
**success** | **bool** |  | 

## Example

```python
from goose_api.models.setup_response import SetupResponse

# TODO update the JSON string below
json = "{}"
# create an instance of SetupResponse from a JSON string
setup_response_instance = SetupResponse.from_json(json)
# print the JSON string representation of the object
print(SetupResponse.to_json())

# convert the object into a dict
setup_response_dict = setup_response_instance.to_dict()
# create an instance of SetupResponse from a dict
setup_response_from_dict = SetupResponse.from_dict(setup_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


