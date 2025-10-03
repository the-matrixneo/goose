# KillJobResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **str** |  | 

## Example

```python
from goose_api.models.kill_job_response import KillJobResponse

# TODO update the JSON string below
json = "{}"
# create an instance of KillJobResponse from a JSON string
kill_job_response_instance = KillJobResponse.from_json(json)
# print the JSON string representation of the object
print(KillJobResponse.to_json())

# convert the object into a dict
kill_job_response_dict = kill_job_response_instance.to_dict()
# create an instance of KillJobResponse from a dict
kill_job_response_from_dict = KillJobResponse.from_dict(kill_job_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


