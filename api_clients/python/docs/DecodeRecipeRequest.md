# DecodeRecipeRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deeplink** | **str** |  | 

## Example

```python
from goose_api.models.decode_recipe_request import DecodeRecipeRequest

# TODO update the JSON string below
json = "{}"
# create an instance of DecodeRecipeRequest from a JSON string
decode_recipe_request_instance = DecodeRecipeRequest.from_json(json)
# print the JSON string representation of the object
print(DecodeRecipeRequest.to_json())

# convert the object into a dict
decode_recipe_request_dict = decode_recipe_request_instance.to_dict()
# create an instance of DecodeRecipeRequest from a dict
decode_recipe_request_from_dict = DecodeRecipeRequest.from_dict(decode_recipe_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


