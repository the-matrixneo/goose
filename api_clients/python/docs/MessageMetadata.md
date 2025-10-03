# MessageMetadata

Metadata for message visibility

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_visible** | **bool** | Whether the message should be included in the agent&#39;s context window | [optional] 
**user_visible** | **bool** | Whether the message should be visible to the user in the UI | [optional] 

## Example

```python
from goose_api.models.message_metadata import MessageMetadata

# TODO update the JSON string below
json = "{}"
# create an instance of MessageMetadata from a JSON string
message_metadata_instance = MessageMetadata.from_json(json)
# print the JSON string representation of the object
print(MessageMetadata.to_json())

# convert the object into a dict
message_metadata_dict = message_metadata_instance.to_dict()
# create an instance of MessageMetadata from a dict
message_metadata_from_dict = MessageMetadata.from_dict(message_metadata_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


