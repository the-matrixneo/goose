# ListRecipeResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipe_manifest_responses** | [**List[RecipeManifestResponse]**](RecipeManifestResponse.md) |  | 

## Example

```python
from goose_api.models.list_recipe_response import ListRecipeResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ListRecipeResponse from a JSON string
list_recipe_response_instance = ListRecipeResponse.from_json(json)
# print the JSON string representation of the object
print(ListRecipeResponse.to_json())

# convert the object into a dict
list_recipe_response_dict = list_recipe_response_instance.to_dict()
# create an instance of ListRecipeResponse from a dict
list_recipe_response_from_dict = ListRecipeResponse.from_dict(list_recipe_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


