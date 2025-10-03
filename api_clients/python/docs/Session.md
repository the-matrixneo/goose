# Session


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accumulated_input_tokens** | **int** |  | [optional] 
**accumulated_output_tokens** | **int** |  | [optional] 
**accumulated_total_tokens** | **int** |  | [optional] 
**conversation** | [**List[Message]**](Message.md) |  | [optional] 
**created_at** | **datetime** |  | 
**description** | **str** |  | 
**extension_data** | **Dict[str, object]** | Extension data containing all extension states Keys are in format \&quot;extension_name.version\&quot; (e.g., \&quot;todo.v0\&quot;) | 
**id** | **str** |  | 
**input_tokens** | **int** |  | [optional] 
**message_count** | **int** |  | 
**output_tokens** | **int** |  | [optional] 
**recipe** | [**Recipe**](Recipe.md) |  | [optional] 
**schedule_id** | **str** |  | [optional] 
**total_tokens** | **int** |  | [optional] 
**updated_at** | **datetime** |  | 
**working_dir** | **str** |  | 

## Example

```python
from goose_api.models.session import Session

# TODO update the JSON string below
json = "{}"
# create an instance of Session from a JSON string
session_instance = Session.from_json(json)
# print the JSON string representation of the object
print(Session.to_json())

# convert the object into a dict
session_dict = session_instance.to_dict()
# create an instance of Session from a dict
session_from_dict = Session.from_dict(session_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


