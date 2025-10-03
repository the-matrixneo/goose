# ScheduleAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createSchedule**](ScheduleAPI.md#createschedule) | **POST** /schedule/create | 
[**deleteSchedule**](ScheduleAPI.md#deleteschedule) | **DELETE** /schedule/delete/{id} | 
[**inspectRunningJob**](ScheduleAPI.md#inspectrunningjob) | **GET** /schedule/{id}/inspect | 
[**killRunningJob**](ScheduleAPI.md#killrunningjob) | **POST** /schedule/{id}/kill | 
[**listSchedules**](ScheduleAPI.md#listschedules) | **GET** /schedule/list | 
[**pauseSchedule**](ScheduleAPI.md#pauseschedule) | **POST** /schedule/{id}/pause | 
[**runNowHandler**](ScheduleAPI.md#runnowhandler) | **POST** /schedule/{id}/run_now | 
[**sessionsHandler**](ScheduleAPI.md#sessionshandler) | **GET** /schedule/{id}/sessions | 
[**unpauseSchedule**](ScheduleAPI.md#unpauseschedule) | **POST** /schedule/{id}/unpause | 
[**updateSchedule**](ScheduleAPI.md#updateschedule) | **PUT** /schedule/{id} | 


# **createSchedule**
```swift
    open class func createSchedule(createScheduleRequest: CreateScheduleRequest, completion: @escaping (_ data: ScheduledJob?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let createScheduleRequest = CreateScheduleRequest(cron: "cron_example", executionMode: "executionMode_example", id: "id_example", recipeSource: "recipeSource_example") // CreateScheduleRequest | 

ScheduleAPI.createSchedule(createScheduleRequest: createScheduleRequest) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
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

# **deleteSchedule**
```swift
    open class func deleteSchedule(id: String, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | ID of the schedule to delete

ScheduleAPI.deleteSchedule(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** | ID of the schedule to delete | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **inspectRunningJob**
```swift
    open class func inspectRunningJob(id: String, completion: @escaping (_ data: InspectJobResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | ID of the schedule to inspect

ScheduleAPI.inspectRunningJob(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** | ID of the schedule to inspect | 

### Return type

[**InspectJobResponse**](InspectJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **killRunningJob**
```swift
    open class func killRunningJob(id: String, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | 

ScheduleAPI.killRunningJob(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** |  | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listSchedules**
```swift
    open class func listSchedules(completion: @escaping (_ data: ListSchedulesResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


ScheduleAPI.listSchedules() { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pauseSchedule**
```swift
    open class func pauseSchedule(id: String, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | ID of the schedule to pause

ScheduleAPI.pauseSchedule(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** | ID of the schedule to pause | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **runNowHandler**
```swift
    open class func runNowHandler(id: String, completion: @escaping (_ data: RunNowResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | ID of the schedule to run

ScheduleAPI.runNowHandler(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** | ID of the schedule to run | 

### Return type

[**RunNowResponse**](RunNowResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sessionsHandler**
```swift
    open class func sessionsHandler(id: String, limit: Int? = nil, completion: @escaping (_ data: [SessionDisplayInfo]?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | ID of the schedule
let limit = 987 // Int |  (optional)

ScheduleAPI.sessionsHandler(id: id, limit: limit) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** | ID of the schedule | 
 **limit** | **Int** |  | [optional] 

### Return type

[**[SessionDisplayInfo]**](SessionDisplayInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **unpauseSchedule**
```swift
    open class func unpauseSchedule(id: String, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | ID of the schedule to unpause

ScheduleAPI.unpauseSchedule(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** | ID of the schedule to unpause | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateSchedule**
```swift
    open class func updateSchedule(id: String, updateScheduleRequest: UpdateScheduleRequest, completion: @escaping (_ data: ScheduledJob?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | ID of the schedule to update
let updateScheduleRequest = UpdateScheduleRequest(cron: "cron_example") // UpdateScheduleRequest | 

ScheduleAPI.updateSchedule(id: id, updateScheduleRequest: updateScheduleRequest) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String** | ID of the schedule to update | 
 **updateScheduleRequest** | [**UpdateScheduleRequest**](UpdateScheduleRequest.md) |  | 

### Return type

[**ScheduledJob**](ScheduledJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

