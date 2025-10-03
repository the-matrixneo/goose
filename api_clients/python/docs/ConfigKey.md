# ConfigKey

Configuration key metadata for provider setup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default** | **str** | Optional default value for the key | [optional] 
**name** | **str** | The name of the configuration key (e.g., \&quot;API_KEY\&quot;) | 
**oauth_flow** | **bool** | Whether this key should be configured using OAuth device code flow When true, the provider&#39;s configure_oauth() method will be called instead of prompting for manual input | 
**required** | **bool** | Whether this key is required for the provider to function | 
**secret** | **bool** | Whether this key should be stored securely (e.g., in keychain) | 

## Example

```python
from goose_api.models.config_key import ConfigKey

# TODO update the JSON string below
json = "{}"
# create an instance of ConfigKey from a JSON string
config_key_instance = ConfigKey.from_json(json)
# print the JSON string representation of the object
print(ConfigKey.to_json())

# convert the object into a dict
config_key_dict = config_key_instance.to_dict()
# create an instance of ConfigKey from a dict
config_key_from_dict = ConfigKey.from_dict(config_key_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


