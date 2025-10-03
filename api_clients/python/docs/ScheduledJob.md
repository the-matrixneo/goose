# ScheduledJob


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cron** | **str** |  | 
**current_session_id** | **str** |  | [optional] 
**currently_running** | **bool** |  | [optional] 
**execution_mode** | **str** |  | [optional] 
**id** | **str** |  | 
**last_run** | **datetime** |  | [optional] 
**paused** | **bool** |  | [optional] 
**process_start_time** | **datetime** |  | [optional] 
**source** | **str** |  | 

## Example

```python
from goose_api.models.scheduled_job import ScheduledJob

# TODO update the JSON string below
json = "{}"
# create an instance of ScheduledJob from a JSON string
scheduled_job_instance = ScheduledJob.from_json(json)
# print the JSON string representation of the object
print(ScheduledJob.to_json())

# convert the object into a dict
scheduled_job_dict = scheduled_job_instance.to_dict()
# create an instance of ScheduledJob from a dict
scheduled_job_from_dict = ScheduledJob.from_dict(scheduled_job_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


