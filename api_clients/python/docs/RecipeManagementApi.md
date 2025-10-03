# goose_api.RecipeManagementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_recipe**](RecipeManagementApi.md#create_recipe) | **POST** /recipes/create | Create a Recipe configuration from the current session
[**decode_recipe**](RecipeManagementApi.md#decode_recipe) | **POST** /recipes/decode | 
[**delete_recipe**](RecipeManagementApi.md#delete_recipe) | **POST** /recipes/delete | 
[**encode_recipe**](RecipeManagementApi.md#encode_recipe) | **POST** /recipes/encode | 
[**list_recipes**](RecipeManagementApi.md#list_recipes) | **GET** /recipes/list | 
[**scan_recipe**](RecipeManagementApi.md#scan_recipe) | **POST** /recipes/scan | 


# **create_recipe**
> CreateRecipeResponse create_recipe(create_recipe_request)

Create a Recipe configuration from the current session

### Example


```python
import goose_api
from goose_api.models.create_recipe_request import CreateRecipeRequest
from goose_api.models.create_recipe_response import CreateRecipeResponse
from goose_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = goose_api.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with goose_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = goose_api.RecipeManagementApi(api_client)
    create_recipe_request = goose_api.CreateRecipeRequest() # CreateRecipeRequest | 

    try:
        # Create a Recipe configuration from the current session
        api_response = api_instance.create_recipe(create_recipe_request)
        print("The response of RecipeManagementApi->create_recipe:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RecipeManagementApi->create_recipe: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_recipe_request** | [**CreateRecipeRequest**](CreateRecipeRequest.md)|  | 

### Return type

[**CreateRecipeResponse**](CreateRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Recipe created successfully |  -  |
**400** | Bad request |  -  |
**412** | Precondition failed - Agent not available |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **decode_recipe**
> DecodeRecipeResponse decode_recipe(decode_recipe_request)

### Example


```python
import goose_api
from goose_api.models.decode_recipe_request import DecodeRecipeRequest
from goose_api.models.decode_recipe_response import DecodeRecipeResponse
from goose_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = goose_api.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with goose_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = goose_api.RecipeManagementApi(api_client)
    decode_recipe_request = goose_api.DecodeRecipeRequest() # DecodeRecipeRequest | 

    try:
        api_response = api_instance.decode_recipe(decode_recipe_request)
        print("The response of RecipeManagementApi->decode_recipe:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RecipeManagementApi->decode_recipe: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **decode_recipe_request** | [**DecodeRecipeRequest**](DecodeRecipeRequest.md)|  | 

### Return type

[**DecodeRecipeResponse**](DecodeRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Recipe decoded successfully |  -  |
**400** | Bad request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_recipe**
> delete_recipe(delete_recipe_request)

### Example


```python
import goose_api
from goose_api.models.delete_recipe_request import DeleteRecipeRequest
from goose_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = goose_api.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with goose_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = goose_api.RecipeManagementApi(api_client)
    delete_recipe_request = goose_api.DeleteRecipeRequest() # DeleteRecipeRequest | 

    try:
        api_instance.delete_recipe(delete_recipe_request)
    except Exception as e:
        print("Exception when calling RecipeManagementApi->delete_recipe: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **delete_recipe_request** | [**DeleteRecipeRequest**](DeleteRecipeRequest.md)|  | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**204** | Recipe deleted successfully |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**404** | Recipe not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **encode_recipe**
> EncodeRecipeResponse encode_recipe(encode_recipe_request)

### Example


```python
import goose_api
from goose_api.models.encode_recipe_request import EncodeRecipeRequest
from goose_api.models.encode_recipe_response import EncodeRecipeResponse
from goose_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = goose_api.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with goose_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = goose_api.RecipeManagementApi(api_client)
    encode_recipe_request = goose_api.EncodeRecipeRequest() # EncodeRecipeRequest | 

    try:
        api_response = api_instance.encode_recipe(encode_recipe_request)
        print("The response of RecipeManagementApi->encode_recipe:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RecipeManagementApi->encode_recipe: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **encode_recipe_request** | [**EncodeRecipeRequest**](EncodeRecipeRequest.md)|  | 

### Return type

[**EncodeRecipeResponse**](EncodeRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Recipe encoded successfully |  -  |
**400** | Bad request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_recipes**
> ListRecipeResponse list_recipes()

### Example


```python
import goose_api
from goose_api.models.list_recipe_response import ListRecipeResponse
from goose_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = goose_api.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with goose_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = goose_api.RecipeManagementApi(api_client)

    try:
        api_response = api_instance.list_recipes()
        print("The response of RecipeManagementApi->list_recipes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RecipeManagementApi->list_recipes: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ListRecipeResponse**](ListRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Get recipe list successfully |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **scan_recipe**
> ScanRecipeResponse scan_recipe(scan_recipe_request)

### Example


```python
import goose_api
from goose_api.models.scan_recipe_request import ScanRecipeRequest
from goose_api.models.scan_recipe_response import ScanRecipeResponse
from goose_api.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = goose_api.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with goose_api.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = goose_api.RecipeManagementApi(api_client)
    scan_recipe_request = goose_api.ScanRecipeRequest() # ScanRecipeRequest | 

    try:
        api_response = api_instance.scan_recipe(scan_recipe_request)
        print("The response of RecipeManagementApi->scan_recipe:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling RecipeManagementApi->scan_recipe: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scan_recipe_request** | [**ScanRecipeRequest**](ScanRecipeRequest.md)|  | 

### Return type

[**ScanRecipeResponse**](ScanRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Recipe scanned successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

