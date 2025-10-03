# SubRecipe


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **str** |  | [optional] 
**name** | **str** |  | 
**path** | **str** |  | 
**sequential_when_repeated** | **bool** |  | [optional] 
**values** | **Dict[str, str]** |  | [optional] 

## Example

```python
from goose_api.models.sub_recipe import SubRecipe

# TODO update the JSON string below
json = "{}"
# create an instance of SubRecipe from a JSON string
sub_recipe_instance = SubRecipe.from_json(json)
# print the JSON string representation of the object
print(SubRecipe.to_json())

# convert the object into a dict
sub_recipe_dict = sub_recipe_instance.to_dict()
# create an instance of SubRecipe from a dict
sub_recipe_from_dict = SubRecipe.from_dict(sub_recipe_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


