# ConfigKey

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_default** | **String** | Optional default value for the key | [optional] 
**name** | **String** | The name of the configuration key (e.g., \&quot;API_KEY\&quot;) | 
**oauthFlow** | **Bool** | Whether this key should be configured using OAuth device code flow When true, the provider&#39;s configure_oauth() method will be called instead of prompting for manual input | 
**_required** | **Bool** | Whether this key is required for the provider to function | 
**secret** | **Bool** | Whether this key should be stored securely (e.g., in keychain) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


