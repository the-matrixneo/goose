# goose_api.SuperRoutesReplyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirm_permission**](SuperRoutesReplyApi.md#confirm_permission) | **POST** /confirm | 


# **confirm_permission**
> object confirm_permission(permission_confirmation_request)

### Example


```python
import goose_api
from goose_api.models.permission_confirmation_request import PermissionConfirmationRequest
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
    api_instance = goose_api.SuperRoutesReplyApi(api_client)
    permission_confirmation_request = goose_api.PermissionConfirmationRequest() # PermissionConfirmationRequest | 

    try:
        api_response = api_instance.confirm_permission(permission_confirmation_request)
        print("The response of SuperRoutesReplyApi->confirm_permission:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesReplyApi->confirm_permission: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **permission_confirmation_request** | [**PermissionConfirmationRequest**](PermissionConfirmationRequest.md)|  | 

### Return type

**object**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Permission action is confirmed |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

