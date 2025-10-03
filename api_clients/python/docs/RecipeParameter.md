# RecipeParameter


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default** | **str** |  | [optional] 
**description** | **str** |  | 
**input_type** | [**RecipeParameterInputType**](RecipeParameterInputType.md) |  | 
**key** | **str** |  | 
**options** | **List[str]** |  | [optional] 
**requirement** | [**RecipeParameterRequirement**](RecipeParameterRequirement.md) |  | 

## Example

```python
from goose_api.models.recipe_parameter import RecipeParameter

# TODO update the JSON string below
json = "{}"
# create an instance of RecipeParameter from a JSON string
recipe_parameter_instance = RecipeParameter.from_json(json)
# print the JSON string representation of the object
print(RecipeParameter.to_json())

# convert the object into a dict
recipe_parameter_dict = recipe_parameter_instance.to_dict()
# create an instance of RecipeParameter from a dict
recipe_parameter_from_dict = RecipeParameter.from_dict(recipe_parameter_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


