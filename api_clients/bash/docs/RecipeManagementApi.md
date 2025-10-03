# RecipeManagementApi

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**createRecipe**](RecipeManagementApi.md#createRecipe) | **POST** /recipes/create | Create a Recipe configuration from the current session
[**decodeRecipe**](RecipeManagementApi.md#decodeRecipe) | **POST** /recipes/decode | 
[**deleteRecipe**](RecipeManagementApi.md#deleteRecipe) | **POST** /recipes/delete | 
[**encodeRecipe**](RecipeManagementApi.md#encodeRecipe) | **POST** /recipes/encode | 
[**listRecipes**](RecipeManagementApi.md#listRecipes) | **GET** /recipes/list | 
[**scanRecipe**](RecipeManagementApi.md#scanRecipe) | **POST** /recipes/scan | 



## createRecipe

Create a Recipe configuration from the current session

### Example

```bash
goose-api createRecipe
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createRecipeRequest** | [**CreateRecipeRequest**](CreateRecipeRequest.md) |  |

### Return type

[**CreateRecipeResponse**](CreateRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decodeRecipe



### Example

```bash
goose-api decodeRecipe
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **decodeRecipeRequest** | [**DecodeRecipeRequest**](DecodeRecipeRequest.md) |  |

### Return type

[**DecodeRecipeResponse**](DecodeRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deleteRecipe



### Example

```bash
goose-api deleteRecipe
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **deleteRecipeRequest** | [**DeleteRecipeRequest**](DeleteRecipeRequest.md) |  |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not Applicable

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encodeRecipe



### Example

```bash
goose-api encodeRecipe
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **encodeRecipeRequest** | [**EncodeRecipeRequest**](EncodeRecipeRequest.md) |  |

### Return type

[**EncodeRecipeResponse**](EncodeRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## listRecipes



### Example

```bash
goose-api listRecipes
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**ListRecipeResponse**](ListRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scanRecipe



### Example

```bash
goose-api scanRecipe
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scanRecipeRequest** | [**ScanRecipeRequest**](ScanRecipeRequest.md) |  |

### Return type

[**ScanRecipeResponse**](ScanRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

