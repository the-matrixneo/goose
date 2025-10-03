# Recipe

A Recipe represents a personalized, user-generated agent configuration that defines specific behaviors and capabilities within the goose system.  # Fields  ## Required Fields * `version` - Semantic version of the Recipe file format (defaults to \"1.0.0\") * `title` - Short, descriptive name of the Recipe * `description` - Detailed description explaining the Recipe's purpose and functionality * `Instructions` - Instructions that defines the Recipe's behavior  ## Optional Fields * `prompt` - the initial prompt to the session to start with * `extensions` - List of extension configurations required by the Recipe * `context` - Supplementary context information for the Recipe * `activities` - Activity labels that appear when loading the Recipe * `author` - Information about the Recipe's creator and metadata * `parameters` - Additional parameters for the Recipe * `response` - Response configuration including JSON schema validation * `retry` - Retry configuration for automated validation and recovery # Example   use goose::recipe::Recipe;  // Using the builder pattern let recipe = Recipe::builder() .title(\"Example Agent\") .description(\"An example Recipe configuration\") .instructions(\"Act as a helpful assistant\") .build() .expect(\"Missing required fields\");  // Or using struct initialization let recipe = Recipe { version: \"1.0.0\".to_string(), title: \"Example Agent\".to_string(), description: \"An example Recipe configuration\".to_string(), instructions: Some(\"Act as a helpful assistant\".to_string()), prompt: None, extensions: None, context: None, activities: None, author: None, settings: None, parameters: None, response: None, sub_recipes: None, retry: None, }; 

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activities** | **List[str]** |  | [optional] 
**author** | [**Author**](Author.md) |  | [optional] 
**context** | **List[str]** |  | [optional] 
**description** | **str** |  | 
**extensions** | [**List[ExtensionConfig]**](ExtensionConfig.md) |  | [optional] 
**instructions** | **str** |  | [optional] 
**parameters** | [**List[RecipeParameter]**](RecipeParameter.md) |  | [optional] 
**prompt** | **str** |  | [optional] 
**response** | [**Response**](Response.md) |  | [optional] 
**retry** | [**RetryConfig**](RetryConfig.md) |  | [optional] 
**settings** | [**Settings**](Settings.md) |  | [optional] 
**sub_recipes** | [**List[SubRecipe]**](SubRecipe.md) |  | [optional] 
**title** | **str** |  | 
**version** | **str** |  | [optional] 

## Example

```python
from goose_api.models.recipe import Recipe

# TODO update the JSON string below
json = "{}"
# create an instance of Recipe from a JSON string
recipe_instance = Recipe.from_json(json)
# print the JSON string representation of the object
print(Recipe.to_json())

# convert the object into a dict
recipe_dict = recipe_instance.to_dict()
# create an instance of Recipe from a dict
recipe_from_dict = Recipe.from_dict(recipe_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


