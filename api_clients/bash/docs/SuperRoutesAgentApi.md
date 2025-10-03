# SuperRoutesAgentApi

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**addSubRecipes**](SuperRoutesAgentApi.md#addSubRecipes) | **POST** /agent/add_sub_recipes | 
[**extendPrompt**](SuperRoutesAgentApi.md#extendPrompt) | **POST** /agent/prompt | 
[**getTools**](SuperRoutesAgentApi.md#getTools) | **GET** /agent/tools | 
[**resumeAgent**](SuperRoutesAgentApi.md#resumeAgent) | **POST** /agent/resume | 
[**startAgent**](SuperRoutesAgentApi.md#startAgent) | **POST** /agent/start | 
[**updateAgentProvider**](SuperRoutesAgentApi.md#updateAgentProvider) | **POST** /agent/update_provider | 
[**updateRouterToolSelector**](SuperRoutesAgentApi.md#updateRouterToolSelector) | **POST** /agent/update_router_tool_selector | 
[**updateSessionConfig**](SuperRoutesAgentApi.md#updateSessionConfig) | **POST** /agent/session_config | 



## addSubRecipes



### Example

```bash
goose-api addSubRecipes
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addSubRecipesRequest** | [**AddSubRecipesRequest**](AddSubRecipesRequest.md) |  |

### Return type

[**AddSubRecipesResponse**](AddSubRecipesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## extendPrompt



### Example

```bash
goose-api extendPrompt
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **extendPromptRequest** | [**ExtendPromptRequest**](ExtendPromptRequest.md) |  |

### Return type

[**ExtendPromptResponse**](ExtendPromptResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getTools



### Example

```bash
goose-api getTools  session_id=value  extension_name=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **string** | Required session ID to scope tools to a specific session | [default to null]
 **extensionName** | **string** | Optional extension name to filter tools | [optional] [default to null]

### Return type

[**array[ToolInfo]**](ToolInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resumeAgent



### Example

```bash
goose-api resumeAgent
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **resumeAgentRequest** | [**ResumeAgentRequest**](ResumeAgentRequest.md) |  |

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## startAgent



### Example

```bash
goose-api startAgent
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **startAgentRequest** | [**StartAgentRequest**](StartAgentRequest.md) |  |

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updateAgentProvider



### Example

```bash
goose-api updateAgentProvider
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **updateProviderRequest** | [**UpdateProviderRequest**](UpdateProviderRequest.md) |  |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updateRouterToolSelector



### Example

```bash
goose-api updateRouterToolSelector
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **updateRouterToolSelectorRequest** | [**UpdateRouterToolSelectorRequest**](UpdateRouterToolSelectorRequest.md) |  |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## updateSessionConfig



### Example

```bash
goose-api updateSessionConfig
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionConfigRequest** | [**SessionConfigRequest**](SessionConfigRequest.md) |  |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

