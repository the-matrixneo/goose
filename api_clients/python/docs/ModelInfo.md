# ModelInfo

Information about a model's capabilities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_limit** | **int** | The maximum context length this model supports | 
**currency** | **str** | Currency for the costs (default: \&quot;$\&quot;) | [optional] 
**input_token_cost** | **float** | Cost per token for input (optional) | [optional] 
**name** | **str** | The name of the model | 
**output_token_cost** | **float** | Cost per token for output (optional) | [optional] 
**supports_cache_control** | **bool** | Whether this model supports cache control | [optional] 

## Example

```python
from goose_api.models.model_info import ModelInfo

# TODO update the JSON string below
json = "{}"
# create an instance of ModelInfo from a JSON string
model_info_instance = ModelInfo.from_json(json)
# print the JSON string representation of the object
print(ModelInfo.to_json())

# convert the object into a dict
model_info_dict = model_info_instance.to_dict()
# create an instance of ModelInfo from a dict
model_info_from_dict = ModelInfo.from_dict(model_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


