# ScanRecipeRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipe** | [**Recipe**](Recipe.md) |  | 

## Example

```python
from goose_api.models.scan_recipe_request import ScanRecipeRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ScanRecipeRequest from a JSON string
scan_recipe_request_instance = ScanRecipeRequest.from_json(json)
# print the JSON string representation of the object
print(ScanRecipeRequest.to_json())

# convert the object into a dict
scan_recipe_request_dict = scan_recipe_request_instance.to_dict()
# create an instance of ScanRecipeRequest from a dict
scan_recipe_request_from_dict = ScanRecipeRequest.from_dict(scan_recipe_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


