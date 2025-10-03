# SessionDisplayInfo


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accumulated_input_tokens** | **int** |  | [optional] 
**accumulated_output_tokens** | **int** |  | [optional] 
**accumulated_total_tokens** | **int** |  | [optional] 
**created_at** | **str** |  | 
**id** | **str** |  | 
**input_tokens** | **int** |  | [optional] 
**message_count** | **int** |  | 
**name** | **str** |  | 
**output_tokens** | **int** |  | [optional] 
**schedule_id** | **str** |  | [optional] 
**total_tokens** | **int** |  | [optional] 
**working_dir** | **str** |  | 

## Example

```python
from goose_api.models.session_display_info import SessionDisplayInfo

# TODO update the JSON string below
json = "{}"
# create an instance of SessionDisplayInfo from a JSON string
session_display_info_instance = SessionDisplayInfo.from_json(json)
# print the JSON string representation of the object
print(SessionDisplayInfo.to_json())

# convert the object into a dict
session_display_info_dict = session_display_info_instance.to_dict()
# create an instance of SessionDisplayInfo from a dict
session_display_info_from_dict = SessionDisplayInfo.from_dict(session_display_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


