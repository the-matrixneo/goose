# UpsertPermissionsQuery


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tool_permissions** | [**List[ToolPermission]**](ToolPermission.md) |  | 

## Example

```python
from goose_api.models.upsert_permissions_query import UpsertPermissionsQuery

# TODO update the JSON string below
json = "{}"
# create an instance of UpsertPermissionsQuery from a JSON string
upsert_permissions_query_instance = UpsertPermissionsQuery.from_json(json)
# print the JSON string representation of the object
print(UpsertPermissionsQuery.to_json())

# convert the object into a dict
upsert_permissions_query_dict = upsert_permissions_query_instance.to_dict()
# create an instance of UpsertPermissionsQuery from a dict
upsert_permissions_query_from_dict = UpsertPermissionsQuery.from_dict(upsert_permissions_query_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


