# goose_api.ContextManagementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**manage_context**](ContextManagementApi.md#manage_context) | **POST** /context/manage | 


# **manage_context**
> ContextManageResponse manage_context(context_manage_request)

### Example


```python
import goose_api
from goose_api.models.context_manage_request import ContextManageRequest
from goose_api.models.context_manage_response import ContextManageResponse
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
    api_instance = goose_api.ContextManagementApi(api_client)
    context_manage_request = goose_api.ContextManageRequest() # ContextManageRequest | 

    try:
        api_response = api_instance.manage_context(context_manage_request)
        print("The response of ContextManagementApi->manage_context:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ContextManagementApi->manage_context: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **context_manage_request** | [**ContextManageRequest**](ContextManageRequest.md)|  | 

### Return type

[**ContextManageResponse**](ContextManageResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Context managed successfully |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**412** | Precondition failed - Agent not available |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

