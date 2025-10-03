# SuperRoutesConfigManagementApi

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**addExtension**](SuperRoutesConfigManagementApi.md#addExtension) | **POST** /config/extensions | 
[**backupConfig**](SuperRoutesConfigManagementApi.md#backupConfig) | **POST** /config/backup | 
[**createCustomProvider**](SuperRoutesConfigManagementApi.md#createCustomProvider) | **POST** /config/custom-providers | 
[**getExtensions**](SuperRoutesConfigManagementApi.md#getExtensions) | **GET** /config/extensions | 
[**getProviderModels**](SuperRoutesConfigManagementApi.md#getProviderModels) | **GET** /config/providers/{name}/models | 
[**initConfig**](SuperRoutesConfigManagementApi.md#initConfig) | **POST** /config/init | 
[**providers**](SuperRoutesConfigManagementApi.md#providers) | **GET** /config/providers | 
[**readAllConfig**](SuperRoutesConfigManagementApi.md#readAllConfig) | **GET** /config | 
[**readConfig**](SuperRoutesConfigManagementApi.md#readConfig) | **POST** /config/read | 
[**recoverConfig**](SuperRoutesConfigManagementApi.md#recoverConfig) | **POST** /config/recover | 
[**removeConfig**](SuperRoutesConfigManagementApi.md#removeConfig) | **POST** /config/remove | 
[**removeCustomProvider**](SuperRoutesConfigManagementApi.md#removeCustomProvider) | **DELETE** /config/custom-providers/{id} | 
[**removeExtension**](SuperRoutesConfigManagementApi.md#removeExtension) | **DELETE** /config/extensions/{name} | 
[**upsertConfig**](SuperRoutesConfigManagementApi.md#upsertConfig) | **POST** /config/upsert | 
[**upsertPermissions**](SuperRoutesConfigManagementApi.md#upsertPermissions) | **POST** /config/permissions | 
[**validateConfig**](SuperRoutesConfigManagementApi.md#validateConfig) | **GET** /config/validate | 



## addExtension



### Example

```bash
goose-api addExtension
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **extensionQuery** | [**ExtensionQuery**](ExtensionQuery.md) |  |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## backupConfig



### Example

```bash
goose-api backupConfig
```

### Parameters

This endpoint does not need any parameter.

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## createCustomProvider



### Example

```bash
goose-api createCustomProvider
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createCustomProviderRequest** | [**CreateCustomProviderRequest**](CreateCustomProviderRequest.md) |  |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getExtensions



### Example

```bash
goose-api getExtensions
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**ExtensionResponse**](ExtensionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## getProviderModels



### Example

```bash
goose-api getProviderModels name=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **string** | Provider name (e.g., openai) | [default to null]

### Return type

**array[string]**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initConfig



### Example

```bash
goose-api initConfig
```

### Parameters

This endpoint does not need any parameter.

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providers



### Example

```bash
goose-api providers
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**array[ProviderDetails]**](ProviderDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## readAllConfig



### Example

```bash
goose-api readAllConfig
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**ConfigResponse**](ConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## readConfig



### Example

```bash
goose-api readConfig
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **configKeyQuery** | [**ConfigKeyQuery**](ConfigKeyQuery.md) |  |

### Return type

[**AnyType**](AnyType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recoverConfig



### Example

```bash
goose-api recoverConfig
```

### Parameters

This endpoint does not need any parameter.

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## removeConfig



### Example

```bash
goose-api removeConfig
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **configKeyQuery** | [**ConfigKeyQuery**](ConfigKeyQuery.md) |  |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## removeCustomProvider



### Example

```bash
goose-api removeCustomProvider id=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **string** |  | [default to null]

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## removeExtension



### Example

```bash
goose-api removeExtension name=value
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **string** |  | [default to null]

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsertConfig



### Example

```bash
goose-api upsertConfig
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **upsertConfigQuery** | [**UpsertConfigQuery**](UpsertConfigQuery.md) |  |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsertPermissions



### Example

```bash
goose-api upsertPermissions
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **upsertPermissionsQuery** | [**UpsertPermissionsQuery**](UpsertPermissionsQuery.md) |  |

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validateConfig



### Example

```bash
goose-api validateConfig
```

### Parameters

This endpoint does not need any parameter.

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

