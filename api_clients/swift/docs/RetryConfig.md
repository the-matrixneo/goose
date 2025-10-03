# RetryConfig

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checks** | [SuccessCheck] | List of success checks to validate recipe completion | 
**maxRetries** | **Int** | Maximum number of retry attempts before giving up | 
**onFailure** | **String** | Optional shell command to run on failure for cleanup | [optional] 
**onFailureTimeoutSeconds** | **Int64** | Timeout in seconds for on_failure commands (default: 600 seconds) | [optional] 
**timeoutSeconds** | **Int64** | Timeout in seconds for individual shell commands (default: 300 seconds) | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


