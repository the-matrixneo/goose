# ResumeAgentRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **str** |  | 

## Example

```python
from goose_api.models.resume_agent_request import ResumeAgentRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ResumeAgentRequest from a JSON string
resume_agent_request_instance = ResumeAgentRequest.from_json(json)
# print the JSON string representation of the object
print(ResumeAgentRequest.to_json())

# convert the object into a dict
resume_agent_request_dict = resume_agent_request_instance.to_dict()
# create an instance of ResumeAgentRequest from a dict
resume_agent_request_from_dict = ResumeAgentRequest.from_dict(resume_agent_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


