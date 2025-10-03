# ProvidersResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**providers** | [**List[ProviderDetails]**](ProviderDetails.md) |  | 

## Example

```python
from goose_api.models.providers_response import ProvidersResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ProvidersResponse from a JSON string
providers_response_instance = ProvidersResponse.from_json(json)
# print the JSON string representation of the object
print(ProvidersResponse.to_json())

# convert the object into a dict
providers_response_dict = providers_response_instance.to_dict()
# create an instance of ProvidersResponse from a dict
providers_response_from_dict = ProvidersResponse.from_dict(providers_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


