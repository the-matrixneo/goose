# ProviderMetadata

Metadata about a provider's configuration requirements and capabilities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_keys** | [**List[ConfigKey]**](ConfigKey.md) | Required configuration keys | 
**default_model** | **str** | The default/recommended model for this provider | 
**description** | **str** | Description of the provider&#39;s capabilities | 
**display_name** | **str** | Display name for the provider in UIs | 
**known_models** | [**List[ModelInfo]**](ModelInfo.md) | A list of currently known models with their capabilities TODO: eventually query the apis directly | 
**model_doc_link** | **str** | Link to the docs where models can be found | 
**name** | **str** | The unique identifier for this provider | 

## Example

```python
from goose_api.models.provider_metadata import ProviderMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of ProviderMetadata from a JSON string
provider_metadata_instance = ProviderMetadata.from_json(json)
# print the JSON string representation of the object
print(ProviderMetadata.to_json())

# convert the object into a dict
provider_metadata_dict = provider_metadata_instance.to_dict()
# create an instance of ProviderMetadata from a dict
provider_metadata_from_dict = ProviderMetadata.from_dict(provider_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


