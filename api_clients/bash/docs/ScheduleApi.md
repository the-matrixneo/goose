# ScheduleApi

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**createSchedule**](ScheduleApi.md#createSchedule) | **POST** /schedule/create | 
[**deleteSchedule**](ScheduleApi.md#deleteSchedule) | **DELETE** /schedule/delete/{id} | 
[**inspectRunningJob**](ScheduleApi.md#inspectRunningJob) | **GET** /schedule/{id}/inspect | 
[**killRunningJob**](ScheduleApi.md#killRunningJob) | **POST** /schedule/{id}/kill | 
[**listSchedules**](ScheduleApi.md#listSchedules) | **GET** /schedule/list | 
[**pauseSchedule**](ScheduleApi.md#pauseSchedule) | **POST** /schedule/{id}/pause | 
[**runNowHandler**](ScheduleApi.md#runNowHandler) | **POST** /schedule/{id}/run_now | 
[**sessionsHandler**](ScheduleApi.md#sessionsHandler) | **GET** /schedule/{id}/sessions | 
[**unpauseSchedule**](ScheduleApi.md#unpauseSchedule) | **POST** /schedule/{id}/unpause | 
[**updateSchedule**](ScheduleApi.md#updateSchedule) | **PUT** /schedule/{id} | 



## createSchedule



### Example

```bash
goose-api createSchedule
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createScheduleRequest** | [**CreateScheduleRequest**](CreateScheduleRequest.md) |  |

### Return type

[**ScheduledJob**](ScheduledJob.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deleteSchedule



### Example

```bash
goose-api deleteSchedule id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** | ID of the schedule to delete | [default to null]

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inspectRunningJob



### Example

```bash
goose-api inspectRunningJob id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** | ID of the schedule to inspect | [default to null]

### Return type

[**InspectJobResponse**](InspectJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## killRunningJob



### Example

```bash
goose-api killRunningJob id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** |  | [default to null]

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## listSchedules



### Example

```bash
goose-api listSchedules
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**ListSchedulesResponse**](ListSchedulesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pauseSchedule



### Example

```bash
goose-api pauseSchedule id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** | ID of the schedule to pause | [default to null]

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## runNowHandler



### Example

```bash
goose-api runNowHandler id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** | ID of the schedule to run | [default to null]

### Return type

[**RunNowResponse**](RunNowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sessionsHandler



### Example

```bash
goose-api sessionsHandler id=value  limit=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** | ID of the schedule | [default to null]
 **limit** | **integer** |  | [optional] [default to null]

### Return type

[**array[SessionDisplayInfo]**](SessionDisplayInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpauseSchedule



### Example

```bash
goose-api unpauseSchedule id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** | ID of the schedule to unpause | [default to null]

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updateSchedule



### Example

```bash
goose-api updateSchedule id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** | ID of the schedule to update | [default to null]
 **updateScheduleRequest** | [**UpdateScheduleRequest**](UpdateScheduleRequest.md) |  |

### Return type

[**ScheduledJob**](ScheduledJob.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

