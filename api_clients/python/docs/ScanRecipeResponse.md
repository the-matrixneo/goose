# ScanRecipeResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_security_warnings** | **bool** |  | 

## Example

```python
from goose_api.models.scan_recipe_response import ScanRecipeResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ScanRecipeResponse from a JSON string
scan_recipe_response_instance = ScanRecipeResponse.from_json(json)
# print the JSON string representation of the object
print(ScanRecipeResponse.to_json())

# convert the object into a dict
scan_recipe_response_dict = scan_recipe_response_instance.to_dict()
# create an instance of ScanRecipeResponse from a dict
scan_recipe_response_from_dict = ScanRecipeResponse.from_dict(scan_recipe_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


