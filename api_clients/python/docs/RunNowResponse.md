# RunNowResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **str** |  | 

## Example

```python
from goose_api.models.run_now_response import RunNowResponse

# TODO update the JSON string below
json = "{}"
# create an instance of RunNowResponse from a JSON string
run_now_response_instance = RunNowResponse.from_json(json)
# print the JSON string representation of the object
print(RunNowResponse.to_json())

# convert the object into a dict
run_now_response_dict = run_now_response_instance.to_dict()
# create an instance of RunNowResponse from a dict
run_now_response_from_dict = RunNowResponse.from_dict(run_now_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


