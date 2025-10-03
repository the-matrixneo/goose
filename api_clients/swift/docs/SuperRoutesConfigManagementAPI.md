# SuperRoutesConfigManagementAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addExtension**](SuperRoutesConfigManagementAPI.md#addextension) | **POST** /config/extensions | 
[**backupConfig**](SuperRoutesConfigManagementAPI.md#backupconfig) | **POST** /config/backup | 
[**createCustomProvider**](SuperRoutesConfigManagementAPI.md#createcustomprovider) | **POST** /config/custom-providers | 
[**getExtensions**](SuperRoutesConfigManagementAPI.md#getextensions) | **GET** /config/extensions | 
[**getProviderModels**](SuperRoutesConfigManagementAPI.md#getprovidermodels) | **GET** /config/providers/{name}/models | 
[**initConfig**](SuperRoutesConfigManagementAPI.md#initconfig) | **POST** /config/init | 
[**providers**](SuperRoutesConfigManagementAPI.md#providers) | **GET** /config/providers | 
[**readAllConfig**](SuperRoutesConfigManagementAPI.md#readallconfig) | **GET** /config | 
[**readConfig**](SuperRoutesConfigManagementAPI.md#readconfig) | **POST** /config/read | 
[**recoverConfig**](SuperRoutesConfigManagementAPI.md#recoverconfig) | **POST** /config/recover | 
[**removeConfig**](SuperRoutesConfigManagementAPI.md#removeconfig) | **POST** /config/remove | 
[**removeCustomProvider**](SuperRoutesConfigManagementAPI.md#removecustomprovider) | **DELETE** /config/custom-providers/{id} | 
[**removeExtension**](SuperRoutesConfigManagementAPI.md#removeextension) | **DELETE** /config/extensions/{name} | 
[**upsertConfig**](SuperRoutesConfigManagementAPI.md#upsertconfig) | **POST** /config/upsert | 
[**upsertPermissions**](SuperRoutesConfigManagementAPI.md#upsertpermissions) | **POST** /config/permissions | 
[**validateConfig**](SuperRoutesConfigManagementAPI.md#validateconfig) | **GET** /config/validate | 


# **addExtension**
```swift
    open class func addExtension(extensionQuery: ExtensionQuery, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let extensionQuery = ExtensionQuery(config: ExtensionConfig(availableTools: ["availableTools_example"], bundled: false, description: "description_example", envKeys: ["envKeys_example"], envs: "TODO", name: "name_example", timeout: 123, type: "type_example", uri: "uri_example", args: ["args_example"], cmd: "cmd_example", displayName: "displayName_example", headers: "TODO", instructions: "instructions_example", tools: [Tool(annotations: Tool_annotations(destructiveHint: false, idempotentHint: false, openWorldHint: false, readOnlyHint: false, title: "title_example"), description: "description_example", icons: [Icon(mimeType: "mimeType_example", sizes: "sizes_example", src: "src_example")], inputSchema: "TODO", name: "name_example", outputSchema: "TODO", title: "title_example")], code: "code_example", dependencies: ["dependencies_example"]), enabled: false, name: "name_example") // ExtensionQuery | 

SuperRoutesConfigManagementAPI.addExtension(extensionQuery: extensionQuery) { (response, error) in
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
 **extensionQuery** | [**ExtensionQuery**](ExtensionQuery.md) |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **backupConfig**
```swift
    open class func backupConfig(completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesConfigManagementAPI.backupConfig() { (response, error) in
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

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createCustomProvider**
```swift
    open class func createCustomProvider(createCustomProviderRequest: CreateCustomProviderRequest, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let createCustomProviderRequest = CreateCustomProviderRequest(apiKey: "apiKey_example", apiUrl: "apiUrl_example", displayName: "displayName_example", models: ["models_example"], providerType: "providerType_example", supportsStreaming: false) // CreateCustomProviderRequest | 

SuperRoutesConfigManagementAPI.createCustomProvider(createCustomProviderRequest: createCustomProviderRequest) { (response, error) in
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
 **createCustomProviderRequest** | [**CreateCustomProviderRequest**](CreateCustomProviderRequest.md) |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getExtensions**
```swift
    open class func getExtensions(completion: @escaping (_ data: ExtensionResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesConfigManagementAPI.getExtensions() { (response, error) in
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

[**ExtensionResponse**](ExtensionResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getProviderModels**
```swift
    open class func getProviderModels(name: String, completion: @escaping (_ data: [String]?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let name = "name_example" // String | Provider name (e.g., openai)

SuperRoutesConfigManagementAPI.getProviderModels(name: name) { (response, error) in
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
 **name** | **String** | Provider name (e.g., openai) | 

### Return type

**[String]**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **initConfig**
```swift
    open class func initConfig(completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesConfigManagementAPI.initConfig() { (response, error) in
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

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **providers**
```swift
    open class func providers(completion: @escaping (_ data: [ProviderDetails]?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesConfigManagementAPI.providers() { (response, error) in
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

[**[ProviderDetails]**](ProviderDetails.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **readAllConfig**
```swift
    open class func readAllConfig(completion: @escaping (_ data: ConfigResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesConfigManagementAPI.readAllConfig() { (response, error) in
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

[**ConfigResponse**](ConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **readConfig**
```swift
    open class func readConfig(configKeyQuery: ConfigKeyQuery, completion: @escaping (_ data: AnyCodable?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let configKeyQuery = ConfigKeyQuery(isSecret: false, key: "key_example") // ConfigKeyQuery | 

SuperRoutesConfigManagementAPI.readConfig(configKeyQuery: configKeyQuery) { (response, error) in
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
 **configKeyQuery** | [**ConfigKeyQuery**](ConfigKeyQuery.md) |  | 

### Return type

**AnyCodable**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **recoverConfig**
```swift
    open class func recoverConfig(completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesConfigManagementAPI.recoverConfig() { (response, error) in
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

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeConfig**
```swift
    open class func removeConfig(configKeyQuery: ConfigKeyQuery, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let configKeyQuery = ConfigKeyQuery(isSecret: false, key: "key_example") // ConfigKeyQuery | 

SuperRoutesConfigManagementAPI.removeConfig(configKeyQuery: configKeyQuery) { (response, error) in
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
 **configKeyQuery** | [**ConfigKeyQuery**](ConfigKeyQuery.md) |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeCustomProvider**
```swift
    open class func removeCustomProvider(id: String, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let id = "id_example" // String | 

SuperRoutesConfigManagementAPI.removeCustomProvider(id: id) { (response, error) in
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

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeExtension**
```swift
    open class func removeExtension(name: String, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let name = "name_example" // String | 

SuperRoutesConfigManagementAPI.removeExtension(name: name) { (response, error) in
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
 **name** | **String** |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **upsertConfig**
```swift
    open class func upsertConfig(upsertConfigQuery: UpsertConfigQuery, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let upsertConfigQuery = UpsertConfigQuery(isSecret: false, key: "key_example", value: 123) // UpsertConfigQuery | 

SuperRoutesConfigManagementAPI.upsertConfig(upsertConfigQuery: upsertConfigQuery) { (response, error) in
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
 **upsertConfigQuery** | [**UpsertConfigQuery**](UpsertConfigQuery.md) |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **upsertPermissions**
```swift
    open class func upsertPermissions(upsertPermissionsQuery: UpsertPermissionsQuery, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let upsertPermissionsQuery = UpsertPermissionsQuery(toolPermissions: [ToolPermission(permission: PermissionLevel(), toolName: "toolName_example")]) // UpsertPermissionsQuery | 

SuperRoutesConfigManagementAPI.upsertPermissions(upsertPermissionsQuery: upsertPermissionsQuery) { (response, error) in
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
 **upsertPermissionsQuery** | [**UpsertPermissionsQuery**](UpsertPermissionsQuery.md) |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **validateConfig**
```swift
    open class func validateConfig(completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesConfigManagementAPI.validateConfig() { (response, error) in
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

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

