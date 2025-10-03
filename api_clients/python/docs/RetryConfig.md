# RetryConfig

Configuration for retry logic in recipe execution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checks** | [**List[SuccessCheck]**](SuccessCheck.md) | List of success checks to validate recipe completion | 
**max_retries** | **int** | Maximum number of retry attempts before giving up | 
**on_failure** | **str** | Optional shell command to run on failure for cleanup | [optional] 
**on_failure_timeout_seconds** | **int** | Timeout in seconds for on_failure commands (default: 600 seconds) | [optional] 
**timeout_seconds** | **int** | Timeout in seconds for individual shell commands (default: 300 seconds) | [optional] 

## Example

```python
from goose_api.models.retry_config import RetryConfig

# TODO update the JSON string below
json = "{}"
# create an instance of RetryConfig from a JSON string
retry_config_instance = RetryConfig.from_json(json)
# print the JSON string representation of the object
print(RetryConfig.to_json())

# convert the object into a dict
retry_config_dict = retry_config_instance.to_dict()
# create an instance of RetryConfig from a dict
retry_config_from_dict = RetryConfig.from_dict(retry_config_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


