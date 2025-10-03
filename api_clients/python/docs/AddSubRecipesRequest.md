# AddSubRecipesRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **str** |  | 
**sub_recipes** | [**List[SubRecipe]**](SubRecipe.md) |  | 

## Example

```python
from goose_api.models.add_sub_recipes_request import AddSubRecipesRequest

# TODO update the JSON string below
json = "{}"
# create an instance of AddSubRecipesRequest from a JSON string
add_sub_recipes_request_instance = AddSubRecipesRequest.from_json(json)
# print the JSON string representation of the object
print(AddSubRecipesRequest.to_json())

# convert the object into a dict
add_sub_recipes_request_dict = add_sub_recipes_request_instance.to_dict()
# create an instance of AddSubRecipesRequest from a dict
add_sub_recipes_request_from_dict = AddSubRecipesRequest.from_dict(add_sub_recipes_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


