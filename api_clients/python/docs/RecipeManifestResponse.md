# RecipeManifestResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**is_global** | **bool** |  | 
**last_modified** | **str** |  | 
**name** | **str** |  | 
**recipe** | [**Recipe**](Recipe.md) |  | 

## Example

```python
from goose_api.models.recipe_manifest_response import RecipeManifestResponse

# TODO update the JSON string below
json = "{}"
# create an instance of RecipeManifestResponse from a JSON string
recipe_manifest_response_instance = RecipeManifestResponse.from_json(json)
# print the JSON string representation of the object
print(RecipeManifestResponse.to_json())

# convert the object into a dict
recipe_manifest_response_dict = recipe_manifest_response_instance.to_dict()
# create an instance of RecipeManifestResponse from a dict
recipe_manifest_response_from_dict = RecipeManifestResponse.from_dict(recipe_manifest_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


