# CreateRecipeRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activities** | **List[str]** |  | [optional] 
**author** | [**AuthorRequest**](AuthorRequest.md) |  | [optional] 
**description** | **str** |  | 
**messages** | [**List[Message]**](Message.md) |  | 
**session_id** | **str** |  | 
**title** | **str** |  | 

## Example

```python
from goose_api.models.create_recipe_request import CreateRecipeRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateRecipeRequest from a JSON string
create_recipe_request_instance = CreateRecipeRequest.from_json(json)
# print the JSON string representation of the object
print(CreateRecipeRequest.to_json())

# convert the object into a dict
create_recipe_request_dict = create_recipe_request_instance.to_dict()
# create an instance of CreateRecipeRequest from a dict
create_recipe_request_from_dict = CreateRecipeRequest.from_dict(create_recipe_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


