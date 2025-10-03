# Message

A message to or from an LLM

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | [**List[MessageContent]**](MessageContent.md) |  | 
**created** | **int** |  | [optional] 
**id** | **str** |  | [optional] 
**metadata** | [**MessageMetadata**](MessageMetadata.md) |  | [optional] 
**role** | **str** |  | 

## Example

```python
from goose_api.models.message import Message

# TODO update the JSON string below
json = "{}"
# create an instance of Message from a JSON string
message_instance = Message.from_json(json)
# print the JSON string representation of the object
print(Message.to_json())

# convert the object into a dict
message_dict = message_instance.to_dict()
# create an instance of Message from a dict
message_from_dict = Message.from_dict(message_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


