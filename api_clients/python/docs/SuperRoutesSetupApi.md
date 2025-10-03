# goose_api.SuperRoutesSetupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**start_openrouter_setup**](SuperRoutesSetupApi.md#start_openrouter_setup) | **POST** /handle_openrouter | 
[**start_tetrate_setup**](SuperRoutesSetupApi.md#start_tetrate_setup) | **POST** /handle_tetrate | 


# **start_openrouter_setup**
> SetupResponse start_openrouter_setup()

### Example


```python
import goose_api
from goose_api.models.setup_response import SetupResponse
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
    api_instance = goose_api.SuperRoutesSetupApi(api_client)

    try:
        api_response = api_instance.start_openrouter_setup()
        print("The response of SuperRoutesSetupApi->start_openrouter_setup:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesSetupApi->start_openrouter_setup: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**SetupResponse**](SetupResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **start_tetrate_setup**
> SetupResponse start_tetrate_setup()

### Example


```python
import goose_api
from goose_api.models.setup_response import SetupResponse
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
    api_instance = goose_api.SuperRoutesSetupApi(api_client)

    try:
        api_response = api_instance.start_tetrate_setup()
        print("The response of SuperRoutesSetupApi->start_tetrate_setup:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesSetupApi->start_tetrate_setup: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**SetupResponse**](SetupResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

