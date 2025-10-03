# DeleteRecipeRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 

## Example

```python
from goose_api.models.delete_recipe_request import DeleteRecipeRequest

# TODO update the JSON string below
json = "{}"
# create an instance of DeleteRecipeRequest from a JSON string
delete_recipe_request_instance = DeleteRecipeRequest.from_json(json)
# print the JSON string representation of the object
print(DeleteRecipeRequest.to_json())

# convert the object into a dict
delete_recipe_request_dict = delete_recipe_request_instance.to_dict()
# create an instance of DeleteRecipeRequest from a dict
delete_recipe_request_from_dict = DeleteRecipeRequest.from_dict(delete_recipe_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


