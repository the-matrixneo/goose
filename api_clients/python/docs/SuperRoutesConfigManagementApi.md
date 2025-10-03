# goose_api.SuperRoutesConfigManagementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_extension**](SuperRoutesConfigManagementApi.md#add_extension) | **POST** /config/extensions | 
[**backup_config**](SuperRoutesConfigManagementApi.md#backup_config) | **POST** /config/backup | 
[**create_custom_provider**](SuperRoutesConfigManagementApi.md#create_custom_provider) | **POST** /config/custom-providers | 
[**get_extensions**](SuperRoutesConfigManagementApi.md#get_extensions) | **GET** /config/extensions | 
[**get_provider_models**](SuperRoutesConfigManagementApi.md#get_provider_models) | **GET** /config/providers/{name}/models | 
[**init_config**](SuperRoutesConfigManagementApi.md#init_config) | **POST** /config/init | 
[**providers**](SuperRoutesConfigManagementApi.md#providers) | **GET** /config/providers | 
[**read_all_config**](SuperRoutesConfigManagementApi.md#read_all_config) | **GET** /config | 
[**read_config**](SuperRoutesConfigManagementApi.md#read_config) | **POST** /config/read | 
[**recover_config**](SuperRoutesConfigManagementApi.md#recover_config) | **POST** /config/recover | 
[**remove_config**](SuperRoutesConfigManagementApi.md#remove_config) | **POST** /config/remove | 
[**remove_custom_provider**](SuperRoutesConfigManagementApi.md#remove_custom_provider) | **DELETE** /config/custom-providers/{id} | 
[**remove_extension**](SuperRoutesConfigManagementApi.md#remove_extension) | **DELETE** /config/extensions/{name} | 
[**upsert_config**](SuperRoutesConfigManagementApi.md#upsert_config) | **POST** /config/upsert | 
[**upsert_permissions**](SuperRoutesConfigManagementApi.md#upsert_permissions) | **POST** /config/permissions | 
[**validate_config**](SuperRoutesConfigManagementApi.md#validate_config) | **GET** /config/validate | 


# **add_extension**
> str add_extension(extension_query)

### Example


```python
import goose_api
from goose_api.models.extension_query import ExtensionQuery
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    extension_query = goose_api.ExtensionQuery() # ExtensionQuery | 

    try:
        api_response = api_instance.add_extension(extension_query)
        print("The response of SuperRoutesConfigManagementApi->add_extension:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->add_extension: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **extension_query** | [**ExtensionQuery**](ExtensionQuery.md)|  | 

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
**200** | Extension added or updated successfully |  -  |
**400** | Invalid request |  -  |
**422** | Could not serialize config.yaml |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **backup_config**
> str backup_config()

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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)

    try:
        api_response = api_instance.backup_config()
        print("The response of SuperRoutesConfigManagementApi->backup_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->backup_config: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Config file backed up |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_custom_provider**
> str create_custom_provider(create_custom_provider_request)

### Example


```python
import goose_api
from goose_api.models.create_custom_provider_request import CreateCustomProviderRequest
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    create_custom_provider_request = goose_api.CreateCustomProviderRequest() # CreateCustomProviderRequest | 

    try:
        api_response = api_instance.create_custom_provider(create_custom_provider_request)
        print("The response of SuperRoutesConfigManagementApi->create_custom_provider:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->create_custom_provider: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_custom_provider_request** | [**CreateCustomProviderRequest**](CreateCustomProviderRequest.md)|  | 

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
**200** | Custom provider created successfully |  -  |
**400** | Invalid request |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_extensions**
> ExtensionResponse get_extensions()

### Example


```python
import goose_api
from goose_api.models.extension_response import ExtensionResponse
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)

    try:
        api_response = api_instance.get_extensions()
        print("The response of SuperRoutesConfigManagementApi->get_extensions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->get_extensions: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ExtensionResponse**](ExtensionResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | All extensions retrieved successfully |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_provider_models**
> List[str] get_provider_models(name)

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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    name = 'name_example' # str | Provider name (e.g., openai)

    try:
        api_response = api_instance.get_provider_models(name)
        print("The response of SuperRoutesConfigManagementApi->get_provider_models:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->get_provider_models: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **str**| Provider name (e.g., openai) | 

### Return type

**List[str]**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Models fetched successfully |  -  |
**400** | Unknown provider, provider not configured, or authentication error |  -  |
**429** | Rate limit exceeded |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **init_config**
> str init_config()

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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)

    try:
        api_response = api_instance.init_config()
        print("The response of SuperRoutesConfigManagementApi->init_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->init_config: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Config initialization check completed |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **providers**
> List[ProviderDetails] providers()

### Example


```python
import goose_api
from goose_api.models.provider_details import ProviderDetails
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)

    try:
        api_response = api_instance.providers()
        print("The response of SuperRoutesConfigManagementApi->providers:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->providers: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**List[ProviderDetails]**](ProviderDetails.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | All configuration values retrieved successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_all_config**
> ConfigResponse read_all_config()

### Example


```python
import goose_api
from goose_api.models.config_response import ConfigResponse
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)

    try:
        api_response = api_instance.read_all_config()
        print("The response of SuperRoutesConfigManagementApi->read_all_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->read_all_config: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ConfigResponse**](ConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | All configuration values retrieved successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_config**
> object read_config(config_key_query)

### Example


```python
import goose_api
from goose_api.models.config_key_query import ConfigKeyQuery
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    config_key_query = goose_api.ConfigKeyQuery() # ConfigKeyQuery | 

    try:
        api_response = api_instance.read_config(config_key_query)
        print("The response of SuperRoutesConfigManagementApi->read_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->read_config: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **config_key_query** | [**ConfigKeyQuery**](ConfigKeyQuery.md)|  | 

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
**200** | Configuration value retrieved successfully |  -  |
**500** | Unable to get the configuration value |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **recover_config**
> str recover_config()

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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)

    try:
        api_response = api_instance.recover_config()
        print("The response of SuperRoutesConfigManagementApi->recover_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->recover_config: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Config recovery attempted |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_config**
> str remove_config(config_key_query)

### Example


```python
import goose_api
from goose_api.models.config_key_query import ConfigKeyQuery
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    config_key_query = goose_api.ConfigKeyQuery() # ConfigKeyQuery | 

    try:
        api_response = api_instance.remove_config(config_key_query)
        print("The response of SuperRoutesConfigManagementApi->remove_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->remove_config: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **config_key_query** | [**ConfigKeyQuery**](ConfigKeyQuery.md)|  | 

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
**200** | Configuration value removed successfully |  -  |
**404** | Configuration key not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_custom_provider**
> str remove_custom_provider(id)

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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    id = 'id_example' # str | 

    try:
        api_response = api_instance.remove_custom_provider(id)
        print("The response of SuperRoutesConfigManagementApi->remove_custom_provider:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->remove_custom_provider: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Custom provider removed successfully |  -  |
**404** | Provider not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_extension**
> str remove_extension(name)

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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    name = 'name_example' # str | 

    try:
        api_response = api_instance.remove_extension(name)
        print("The response of SuperRoutesConfigManagementApi->remove_extension:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->remove_extension: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **str**|  | 

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Extension removed successfully |  -  |
**404** | Extension not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **upsert_config**
> str upsert_config(upsert_config_query)

### Example


```python
import goose_api
from goose_api.models.upsert_config_query import UpsertConfigQuery
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    upsert_config_query = goose_api.UpsertConfigQuery() # UpsertConfigQuery | 

    try:
        api_response = api_instance.upsert_config(upsert_config_query)
        print("The response of SuperRoutesConfigManagementApi->upsert_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->upsert_config: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **upsert_config_query** | [**UpsertConfigQuery**](UpsertConfigQuery.md)|  | 

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
**200** | Configuration value upserted successfully |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **upsert_permissions**
> str upsert_permissions(upsert_permissions_query)

### Example


```python
import goose_api
from goose_api.models.upsert_permissions_query import UpsertPermissionsQuery
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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)
    upsert_permissions_query = goose_api.UpsertPermissionsQuery() # UpsertPermissionsQuery | 

    try:
        api_response = api_instance.upsert_permissions(upsert_permissions_query)
        print("The response of SuperRoutesConfigManagementApi->upsert_permissions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->upsert_permissions: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **upsert_permissions_query** | [**UpsertPermissionsQuery**](UpsertPermissionsQuery.md)|  | 

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
**200** | Permission update completed |  -  |
**400** | Invalid request |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validate_config**
> str validate_config()

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
    api_instance = goose_api.SuperRoutesConfigManagementApi(api_client)

    try:
        api_response = api_instance.validate_config()
        print("The response of SuperRoutesConfigManagementApi->validate_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SuperRoutesConfigManagementApi->validate_config: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

**str**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Config validation result |  -  |
**422** | Config file is corrupted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

