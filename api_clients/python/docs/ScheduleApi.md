# goose_api.ScheduleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_schedule**](ScheduleApi.md#create_schedule) | **POST** /schedule/create | 
[**delete_schedule**](ScheduleApi.md#delete_schedule) | **DELETE** /schedule/delete/{id} | 
[**inspect_running_job**](ScheduleApi.md#inspect_running_job) | **GET** /schedule/{id}/inspect | 
[**kill_running_job**](ScheduleApi.md#kill_running_job) | **POST** /schedule/{id}/kill | 
[**list_schedules**](ScheduleApi.md#list_schedules) | **GET** /schedule/list | 
[**pause_schedule**](ScheduleApi.md#pause_schedule) | **POST** /schedule/{id}/pause | 
[**run_now_handler**](ScheduleApi.md#run_now_handler) | **POST** /schedule/{id}/run_now | 
[**sessions_handler**](ScheduleApi.md#sessions_handler) | **GET** /schedule/{id}/sessions | 
[**unpause_schedule**](ScheduleApi.md#unpause_schedule) | **POST** /schedule/{id}/unpause | 
[**update_schedule**](ScheduleApi.md#update_schedule) | **PUT** /schedule/{id} | 


# **create_schedule**
> ScheduledJob create_schedule(create_schedule_request)

### Example


```python
import goose_api
from goose_api.models.create_schedule_request import CreateScheduleRequest
from goose_api.models.scheduled_job import ScheduledJob
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
    api_instance = goose_api.ScheduleApi(api_client)
    create_schedule_request = goose_api.CreateScheduleRequest() # CreateScheduleRequest | 

    try:
        api_response = api_instance.create_schedule(create_schedule_request)
        print("The response of ScheduleApi->create_schedule:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ScheduleApi->create_schedule: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_schedule_request** | [**CreateScheduleRequest**](CreateScheduleRequest.md)|  | 

### Return type

[**ScheduledJob**](ScheduledJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Scheduled job created successfully |  -  |
**400** | Invalid cron expression or recipe file |  -  |
**409** | Job ID already exists |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_schedule**
> delete_schedule(id)

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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | ID of the schedule to delete

    try:
        api_instance.delete_schedule(id)
    except Exception as e:
        print("Exception when calling ScheduleApi->delete_schedule: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| ID of the schedule to delete | 

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
**204** | Scheduled job deleted successfully |  -  |
**404** | Scheduled job not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **inspect_running_job**
> InspectJobResponse inspect_running_job(id)

### Example


```python
import goose_api
from goose_api.models.inspect_job_response import InspectJobResponse
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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | ID of the schedule to inspect

    try:
        api_response = api_instance.inspect_running_job(id)
        print("The response of ScheduleApi->inspect_running_job:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ScheduleApi->inspect_running_job: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| ID of the schedule to inspect | 

### Return type

[**InspectJobResponse**](InspectJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Running job information |  -  |
**404** | Scheduled job not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **kill_running_job**
> kill_running_job(id)

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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | 

    try:
        api_instance.kill_running_job(id)
    except Exception as e:
        print("Exception when calling ScheduleApi->kill_running_job: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**|  | 

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
**200** | Running job killed successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_schedules**
> ListSchedulesResponse list_schedules()

### Example


```python
import goose_api
from goose_api.models.list_schedules_response import ListSchedulesResponse
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
    api_instance = goose_api.ScheduleApi(api_client)

    try:
        api_response = api_instance.list_schedules()
        print("The response of ScheduleApi->list_schedules:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ScheduleApi->list_schedules: %s\n" % e)
```



### Parameters

This endpoint does not need any parameter.

### Return type

[**ListSchedulesResponse**](ListSchedulesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A list of scheduled jobs |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pause_schedule**
> pause_schedule(id)

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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | ID of the schedule to pause

    try:
        api_instance.pause_schedule(id)
    except Exception as e:
        print("Exception when calling ScheduleApi->pause_schedule: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| ID of the schedule to pause | 

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
**204** | Scheduled job paused successfully |  -  |
**400** | Cannot pause a currently running job |  -  |
**404** | Scheduled job not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **run_now_handler**
> RunNowResponse run_now_handler(id)

### Example


```python
import goose_api
from goose_api.models.run_now_response import RunNowResponse
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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | ID of the schedule to run

    try:
        api_response = api_instance.run_now_handler(id)
        print("The response of ScheduleApi->run_now_handler:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ScheduleApi->run_now_handler: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| ID of the schedule to run | 

### Return type

[**RunNowResponse**](RunNowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Scheduled job triggered successfully, returns new session ID |  -  |
**404** | Scheduled job not found |  -  |
**500** | Internal server error when trying to run the job |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sessions_handler**
> List[SessionDisplayInfo] sessions_handler(id, limit=limit)

### Example


```python
import goose_api
from goose_api.models.session_display_info import SessionDisplayInfo
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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | ID of the schedule
    limit = 56 # int |  (optional)

    try:
        api_response = api_instance.sessions_handler(id, limit=limit)
        print("The response of ScheduleApi->sessions_handler:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ScheduleApi->sessions_handler: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| ID of the schedule | 
 **limit** | **int**|  | [optional] 

### Return type

[**List[SessionDisplayInfo]**](SessionDisplayInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | A list of session display info |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **unpause_schedule**
> unpause_schedule(id)

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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | ID of the schedule to unpause

    try:
        api_instance.unpause_schedule(id)
    except Exception as e:
        print("Exception when calling ScheduleApi->unpause_schedule: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| ID of the schedule to unpause | 

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
**204** | Scheduled job unpaused successfully |  -  |
**404** | Scheduled job not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_schedule**
> ScheduledJob update_schedule(id, update_schedule_request)

### Example


```python
import goose_api
from goose_api.models.scheduled_job import ScheduledJob
from goose_api.models.update_schedule_request import UpdateScheduleRequest
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
    api_instance = goose_api.ScheduleApi(api_client)
    id = 'id_example' # str | ID of the schedule to update
    update_schedule_request = goose_api.UpdateScheduleRequest() # UpdateScheduleRequest | 

    try:
        api_response = api_instance.update_schedule(id, update_schedule_request)
        print("The response of ScheduleApi->update_schedule:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ScheduleApi->update_schedule: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **str**| ID of the schedule to update | 
 **update_schedule_request** | [**UpdateScheduleRequest**](UpdateScheduleRequest.md)|  | 

### Return type

[**ScheduledJob**](ScheduledJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Scheduled job updated successfully |  -  |
**400** | Cannot update a currently running job or invalid request |  -  |
**404** | Scheduled job not found |  -  |
**500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

