# ProviderDetails


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_configured** | **bool** |  | 
**metadata** | [**ProviderMetadata**](ProviderMetadata.md) |  | 
**name** | **str** |  | 

## Example

```python
from goose_api.models.provider_details import ProviderDetails

# TODO update the JSON string below
json = "{}"
# create an instance of ProviderDetails from a JSON string
provider_details_instance = ProviderDetails.from_json(json)
# print the JSON string representation of the object
print(ProviderDetails.to_json())

# convert the object into a dict
provider_details_dict = provider_details_instance.to_dict()
# create an instance of ProviderDetails from a dict
provider_details_from_dict = ProviderDetails.from_dict(provider_details_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


