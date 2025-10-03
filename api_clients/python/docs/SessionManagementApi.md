# goose_api.SessionManagementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_session**](SessionManagementApi.md#delete_session) | **DELETE** /sessions/{session_id} | 
[**get_session**](SessionManagementApi.md#get_session) | **GET** /sessions/{session_id} | 
[**get_session_insights**](SessionManagementApi.md#get_session_insights) | **GET** /sessions/insights | 
[**list_sessions**](SessionManagementApi.md#list_sessions) | **GET** /sessions | 
[**update_session_description**](SessionManagementApi.md#update_session_description) | **PUT** /sessions/{session_id}/description | 


# **delete_session**
> delete_session(session_id)

### Example


```python
import goose_api
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
    api_instance = goose_api.SessionManagementApi(api_client)
    session_id = 'session_id_example' # str | Unique identifier for the session

    try:
        api_instance.delete_session(session_id)
    except Exception as e:
        print("Exception when calling SessionManagementApi->delete_session: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **session_id** | **str**| Unique identifier for the session | 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Session deleted successfully |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**404** | Session not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_session**
> Session get_session(session_id)

### Example


```python
import goose_api
from goose_api.models.session import Session
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
    api_instance = goose_api.SessionManagementApi(api_client)
    session_id = 'session_id_example' # str | Unique identifier for the session

    try:
        api_response = api_instance.get_session(session_id)
        print("The response of SessionManagementApi->get_session:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SessionManagementApi->get_session: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **session_id** | **str**| Unique identifier for the session | 

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Session history retrieved successfully |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**404** | Session not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_session_insights**
> SessionInsights get_session_insights()

### Example


```python
import goose_api
from goose_api.models.session_insights import SessionInsights
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
    api_instance = goose_api.SessionManagementApi(api_client)

    try:
        api_response = api_instance.get_session_insights()
        print("The response of SessionManagementApi->get_session_insights:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SessionManagementApi->get_session_insights: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**SessionInsights**](SessionInsights.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Session insights retrieved successfully |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_sessions**
> SessionListResponse list_sessions()

### Example


```python
import goose_api
from goose_api.models.session_list_response import SessionListResponse
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
    api_instance = goose_api.SessionManagementApi(api_client)

    try:
        api_response = api_instance.list_sessions()
        print("The response of SessionManagementApi->list_sessions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SessionManagementApi->list_sessions: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**SessionListResponse**](SessionListResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List of available sessions retrieved successfully |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_session_description**
> update_session_description(session_id, update_session_description_request)

### Example


```python
import goose_api
from goose_api.models.update_session_description_request import UpdateSessionDescriptionRequest
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
    api_instance = goose_api.SessionManagementApi(api_client)
    session_id = 'session_id_example' # str | Unique identifier for the session
    update_session_description_request = goose_api.UpdateSessionDescriptionRequest() # UpdateSessionDescriptionRequest | 

    try:
        api_instance.update_session_description(session_id, update_session_description_request)
    except Exception as e:
        print("Exception when calling SessionManagementApi->update_session_description: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **session_id** | **str**| Unique identifier for the session | 
 **update_session_description_request** | [**UpdateSessionDescriptionRequest**](UpdateSessionDescriptionRequest.md)|  | 

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
**200** | Session description updated successfully |  -  |
**400** | Bad request - Description too long (max 200 characters) |  -  |
**401** | Unauthorized - Invalid or missing API key |  -  |
**404** | Session not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

