# goose_api.SuperRoutesAgentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_sub_recipes**](SuperRoutesAgentApi.md#add_sub_recipes) | **POST** /agent/add_sub_recipes | 
[**extend_prompt**](SuperRoutesAgentApi.md#extend_prompt) | **POST** /agent/prompt | 
[**get_tools**](SuperRoutesAgentApi.md#get_tools) | **GET** /agent/tools | 
[**resume_agent**](SuperRoutesAgentApi.md#resume_agent) | **POST** /agent/resume | 
[**start_agent**](SuperRoutesAgentApi.md#start_agent) | **POST** /agent/start | 
[**update_agent_provider**](SuperRoutesAgentApi.md#update_agent_provider) | **POST** /agent/update_provider | 
[**update_router_tool_selector**](SuperRoutesAgentApi.md#update_router_tool_selector) | **POST** /agent/update_router_tool_selector | 
[**update_session_config**](SuperRoutesAgentApi.md#update_session_config) | **POST** /agent/session_config | 


# **add_sub_recipes**
> AddSubRecipesResponse add_sub_recipes(add_sub_recipes_request)

### Example


```python
import goose_api
from goose_api.models.add_sub_recipes_request import AddSubRecipesRequest
from goose_api.models.add_sub_recipes_response import AddSubRecipesResponse
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    add_sub_recipes_request = goose_api.AddSubRecipesRequest() # AddSubRecipesRequest | 

    try:
        api_response = api_instance.add_sub_recipes(add_sub_recipes_request)
        print("The response of SuperRoutesAgentApi->add_sub_recipes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->add_sub_recipes: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **add_sub_recipes_request** | [**AddSubRecipesRequest**](AddSubRecipesRequest.md)|  | 

### Return type

[**AddSubRecipesResponse**](AddSubRecipesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Added sub recipes to agent successfully |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**424** | Agent not initialized |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **extend_prompt**
> ExtendPromptResponse extend_prompt(extend_prompt_request)

### Example


```python
import goose_api
from goose_api.models.extend_prompt_request import ExtendPromptRequest
from goose_api.models.extend_prompt_response import ExtendPromptResponse
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    extend_prompt_request = goose_api.ExtendPromptRequest() # ExtendPromptRequest | 

    try:
        api_response = api_instance.extend_prompt(extend_prompt_request)
        print("The response of SuperRoutesAgentApi->extend_prompt:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->extend_prompt: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **extend_prompt_request** | [**ExtendPromptRequest**](ExtendPromptRequest.md)|  | 

### Return type

[**ExtendPromptResponse**](ExtendPromptResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Extended system prompt successfully |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**424** | Agent not initialized |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tools**
> List[ToolInfo] get_tools(session_id, extension_name=extension_name)

### Example


```python
import goose_api
from goose_api.models.tool_info import ToolInfo
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    session_id = 'session_id_example' # str | Required session ID to scope tools to a specific session
    extension_name = 'extension_name_example' # str | Optional extension name to filter tools (optional)

    try:
        api_response = api_instance.get_tools(session_id, extension_name=extension_name)
        print("The response of SuperRoutesAgentApi->get_tools:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->get_tools: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **session_id** | **str**| Required session ID to scope tools to a specific session | 
 **extension_name** | **str**| Optional extension name to filter tools | [optional] 

### Return type

[**List[ToolInfo]**](ToolInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tools retrieved successfully |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**424** | Agent not initialized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **resume_agent**
> Session resume_agent(resume_agent_request)

### Example


```python
import goose_api
from goose_api.models.resume_agent_request import ResumeAgentRequest
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    resume_agent_request = goose_api.ResumeAgentRequest() # ResumeAgentRequest | 

    try:
        api_response = api_instance.resume_agent(resume_agent_request)
        print("The response of SuperRoutesAgentApi->resume_agent:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->resume_agent: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **resume_agent_request** | [**ResumeAgentRequest**](ResumeAgentRequest.md)|  | 

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Agent started successfully |  -  |
**400** | Bad request - invalid working directory |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **start_agent**
> Session start_agent(start_agent_request)

### Example


```python
import goose_api
from goose_api.models.session import Session
from goose_api.models.start_agent_request import StartAgentRequest
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    start_agent_request = goose_api.StartAgentRequest() # StartAgentRequest | 

    try:
        api_response = api_instance.start_agent(start_agent_request)
        print("The response of SuperRoutesAgentApi->start_agent:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->start_agent: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **start_agent_request** | [**StartAgentRequest**](StartAgentRequest.md)|  | 

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Agent started successfully |  -  |
**400** | Bad request - invalid working directory |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_agent_provider**
> update_agent_provider(update_provider_request)

### Example


```python
import goose_api
from goose_api.models.update_provider_request import UpdateProviderRequest
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    update_provider_request = goose_api.UpdateProviderRequest() # UpdateProviderRequest | 

    try:
        api_instance.update_agent_provider(update_provider_request)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->update_agent_provider: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_provider_request** | [**UpdateProviderRequest**](UpdateProviderRequest.md)|  | 

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
**200** | Provider updated successfully |  -  |
**400** | Bad request - missing or invalid parameters |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**424** | Agent not initialized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_router_tool_selector**
> str update_router_tool_selector(update_router_tool_selector_request)

### Example


```python
import goose_api
from goose_api.models.update_router_tool_selector_request import UpdateRouterToolSelectorRequest
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    update_router_tool_selector_request = goose_api.UpdateRouterToolSelectorRequest() # UpdateRouterToolSelectorRequest | 

    try:
        api_response = api_instance.update_router_tool_selector(update_router_tool_selector_request)
        print("The response of SuperRoutesAgentApi->update_router_tool_selector:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->update_router_tool_selector: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **update_router_tool_selector_request** | [**UpdateRouterToolSelectorRequest**](UpdateRouterToolSelectorRequest.md)|  | 

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Tool selection strategy updated successfully |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**424** | Agent not initialized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_session_config**
> str update_session_config(session_config_request)

### Example


```python
import goose_api
from goose_api.models.session_config_request import SessionConfigRequest
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
    api_instance = goose_api.SuperRoutesAgentApi(api_client)
    session_config_request = goose_api.SessionConfigRequest() # SessionConfigRequest | 

    try:
        api_response = api_instance.update_session_config(session_config_request)
        print("The response of SuperRoutesAgentApi->update_session_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesAgentApi->update_session_config: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **session_config_request** | [**SessionConfigRequest**](SessionConfigRequest.md)|  | 

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Session config updated successfully |  -  |
**401** | Unauthorized - invalid secret key |  -  |
**424** | Agent not initialized |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

