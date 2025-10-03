# InspectJobResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**process_start_time** | **str** |  | [optional] 
**running_duration_seconds** | **int** |  | [optional] 
**session_id** | **str** |  | [optional] 

## Example

```python
from goose_api.models.inspect_job_response import InspectJobResponse

# TODO update the JSON string below
json = "{}"
# create an instance of InspectJobResponse from a JSON string
inspect_job_response_instance = InspectJobResponse.from_json(json)
# print the JSON string representation of the object
print(InspectJobResponse.to_json())

# convert the object into a dict
inspect_job_response_dict = inspect_job_response_instance.to_dict()
# create an instance of InspectJobResponse from a dict
inspect_job_response_from_dict = InspectJobResponse.from_dict(inspect_job_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


