# DecodeRecipeResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipe** | [**Recipe**](Recipe.md) |  | 

## Example

```python
from goose_api.models.decode_recipe_response import DecodeRecipeResponse

# TODO update the JSON string below
json = "{}"
# create an instance of DecodeRecipeResponse from a JSON string
decode_recipe_response_instance = DecodeRecipeResponse.from_json(json)
# print the JSON string representation of the object
print(DecodeRecipeResponse.to_json())

# convert the object into a dict
decode_recipe_response_dict = decode_recipe_response_instance.to_dict()
# create an instance of DecodeRecipeResponse from a dict
decode_recipe_response_from_dict = DecodeRecipeResponse.from_dict(decode_recipe_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


