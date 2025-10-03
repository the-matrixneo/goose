# EncodeRecipeRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipe** | [**Recipe**](Recipe.md) |  | 

## Example

```python
from goose_api.models.encode_recipe_request import EncodeRecipeRequest

# TODO update the JSON string below
json = "{}"
# create an instance of EncodeRecipeRequest from a JSON string
encode_recipe_request_instance = EncodeRecipeRequest.from_json(json)
# print the JSON string representation of the object
print(EncodeRecipeRequest.to_json())

# convert the object into a dict
encode_recipe_request_dict = encode_recipe_request_instance.to_dict()
# create an instance of EncodeRecipeRequest from a dict
encode_recipe_request_from_dict = EncodeRecipeRequest.from_dict(encode_recipe_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


