# SessionManagementApi

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**deleteSession**](SessionManagementApi.md#deleteSession) | **DELETE** /sessions/{session_id} | 
[**getSession**](SessionManagementApi.md#getSession) | **GET** /sessions/{session_id} | 
[**getSessionInsights**](SessionManagementApi.md#getSessionInsights) | **GET** /sessions/insights | 
[**listSessions**](SessionManagementApi.md#listSessions) | **GET** /sessions | 
[**updateSessionDescription**](SessionManagementApi.md#updateSessionDescription) | **PUT** /sessions/{session_id}/description | 



## deleteSession



### Example

```bash
goose-api deleteSession session_id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **string** | Unique identifier for the session | [default to null]

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getSession



### Example

```bash
goose-api getSession session_id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **string** | Unique identifier for the session | [default to null]

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getSessionInsights



### Example

```bash
goose-api getSessionInsights
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**SessionInsights**](SessionInsights.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## listSessions



### Example

```bash
goose-api listSessions
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**SessionListResponse**](SessionListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updateSessionDescription



### Example

```bash
goose-api updateSessionDescription session_id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **string** | Unique identifier for the session | [default to null]
 **updateSessionDescriptionRequest** | [**UpdateSessionDescriptionRequest**](UpdateSessionDescriptionRequest.md) |  |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

