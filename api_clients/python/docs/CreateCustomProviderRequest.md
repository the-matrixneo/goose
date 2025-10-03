# CreateCustomProviderRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | **str** |  | 
**api_url** | **str** |  | 
**display_name** | **str** |  | 
**models** | **List[str]** |  | 
**provider_type** | **str** |  | 
**supports_streaming** | **bool** |  | [optional] 

## Example

```python
from goose_api.models.create_custom_provider_request import CreateCustomProviderRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateCustomProviderRequest from a JSON string
create_custom_provider_request_instance = CreateCustomProviderRequest.from_json(json)
# print the JSON string representation of the object
print(CreateCustomProviderRequest.to_json())

# convert the object into a dict
create_custom_provider_request_dict = create_custom_provider_request_instance.to_dict()
# create an instance of CreateCustomProviderRequest from a dict
create_custom_provider_request_from_dict = CreateCustomProviderRequest.from_dict(create_custom_provider_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


