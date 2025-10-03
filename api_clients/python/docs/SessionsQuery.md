# SessionsQuery


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limit** | **int** |  | [optional] 

## Example

```python
from goose_api.models.sessions_query import SessionsQuery

# TODO update the JSON string below
json = "{}"
# create an instance of SessionsQuery from a JSON string
sessions_query_instance = SessionsQuery.from_json(json)
# print the JSON string representation of the object
print(SessionsQuery.to_json())

# convert the object into a dict
sessions_query_dict = sessions_query_instance.to_dict()
# create an instance of SessionsQuery from a dict
sessions_query_from_dict = SessionsQuery.from_dict(sessions_query_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


