# AuthorRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contact** | **str** |  | [optional] 
**metadata** | **str** |  | [optional] 

## Example

```python
from goose_api.models.author_request import AuthorRequest

# TODO update the JSON string below
json = "{}"
# create an instance of AuthorRequest from a JSON string
author_request_instance = AuthorRequest.from_json(json)
# print the JSON string representation of the object
print(AuthorRequest.to_json())

# convert the object into a dict
author_request_dict = author_request_instance.to_dict()
# create an instance of AuthorRequest from a dict
author_request_from_dict = AuthorRequest.from_dict(author_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


