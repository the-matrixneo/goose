# SessionInsights


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_sessions** | **int** | Total number of sessions | 
**total_tokens** | **int** | Total tokens used across all sessions | 

## Example

```python
from goose_api.models.session_insights import SessionInsights

# TODO update the JSON string below
json = "{}"
# create an instance of SessionInsights from a JSON string
session_insights_instance = SessionInsights.from_json(json)
# print the JSON string representation of the object
print(SessionInsights.to_json())

# convert the object into a dict
session_insights_dict = session_insights_instance.to_dict()
# create an instance of SessionInsights from a dict
session_insights_from_dict = SessionInsights.from_dict(session_insights_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


