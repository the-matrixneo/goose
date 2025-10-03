# StartAgentRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipe** | [**Recipe**](Recipe.md) |  | [optional] 
**working_dir** | **str** |  | 

## Example

```python
from goose_api.models.start_agent_request import StartAgentRequest

# TODO update the JSON string below
json = "{}"
# create an instance of StartAgentRequest from a JSON string
start_agent_request_instance = StartAgentRequest.from_json(json)
# print the JSON string representation of the object
print(StartAgentRequest.to_json())

# convert the object into a dict
start_agent_request_dict = start_agent_request_instance.to_dict()
# create an instance of StartAgentRequest from a dict
start_agent_request_from_dict = StartAgentRequest.from_dict(start_agent_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


