# RecipeManagementAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createRecipe**](RecipeManagementAPI.md#createrecipe) | **POST** /recipes/create | Create a Recipe configuration from the current session
[**decodeRecipe**](RecipeManagementAPI.md#decoderecipe) | **POST** /recipes/decode | 
[**deleteRecipe**](RecipeManagementAPI.md#deleterecipe) | **POST** /recipes/delete | 
[**encodeRecipe**](RecipeManagementAPI.md#encoderecipe) | **POST** /recipes/encode | 
[**listRecipes**](RecipeManagementAPI.md#listrecipes) | **GET** /recipes/list | 
[**scanRecipe**](RecipeManagementAPI.md#scanrecipe) | **POST** /recipes/scan | 


# **createRecipe**
```swift
    open class func createRecipe(createRecipeRequest: CreateRecipeRequest, completion: @escaping (_ data: CreateRecipeResponse?, _ error: Error?) -> Void)
```

Create a Recipe configuration from the current session

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let createRecipeRequest = CreateRecipeRequest(activities: ["activities_example"], author: AuthorRequest(contact: "contact_example", metadata: "metadata_example"), description: "description_example", messages: [Message(content: [MessageContent(meta: "TODO", annotations: EmbeddedResource_annotations(audience: ["audience_example"], lastModified: Date(), priority: 123), text: "text_example", type: "type_example", data: "data_example", mimeType: "mimeType_example", id: "id_example", toolCall: 123, toolResult: 123, arguments: "TODO", prompt: "prompt_example", toolName: "toolName_example", signature: "signature_example", thinking: "thinking_example", msg: "msg_example")], created: 123, id: "id_example", metadata: MessageMetadata(agentVisible: false, userVisible: false), role: "role_example")], sessionId: "sessionId_example", title: "title_example") // CreateRecipeRequest | 

// Create a Recipe configuration from the current session
RecipeManagementAPI.createRecipe(createRecipeRequest: createRecipeRequest) { (response, error) in
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
 **createRecipeRequest** | [**CreateRecipeRequest**](CreateRecipeRequest.md) |  | 

### Return type

[**CreateRecipeResponse**](CreateRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **decodeRecipe**
```swift
    open class func decodeRecipe(decodeRecipeRequest: DecodeRecipeRequest, completion: @escaping (_ data: DecodeRecipeResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let decodeRecipeRequest = DecodeRecipeRequest(deeplink: "deeplink_example") // DecodeRecipeRequest | 

RecipeManagementAPI.decodeRecipe(decodeRecipeRequest: decodeRecipeRequest) { (response, error) in
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
 **decodeRecipeRequest** | [**DecodeRecipeRequest**](DecodeRecipeRequest.md) |  | 

### Return type

[**DecodeRecipeResponse**](DecodeRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteRecipe**
```swift
    open class func deleteRecipe(deleteRecipeRequest: DeleteRecipeRequest, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let deleteRecipeRequest = DeleteRecipeRequest(id: "id_example") // DeleteRecipeRequest | 

RecipeManagementAPI.deleteRecipe(deleteRecipeRequest: deleteRecipeRequest) { (response, error) in
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
 **deleteRecipeRequest** | [**DeleteRecipeRequest**](DeleteRecipeRequest.md) |  | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **encodeRecipe**
```swift
    open class func encodeRecipe(encodeRecipeRequest: EncodeRecipeRequest, completion: @escaping (_ data: EncodeRecipeResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let encodeRecipeRequest = EncodeRecipeRequest(recipe: Recipe(activities: ["activities_example"], author: Author(contact: "contact_example", metadata: "metadata_example"), context: ["context_example"], description: "description_example", extensions: [ExtensionConfig(availableTools: ["availableTools_example"], bundled: false, description: "description_example", envKeys: ["envKeys_example"], envs: "TODO", name: "name_example", timeout: 123, type: "type_example", uri: "uri_example", args: ["args_example"], cmd: "cmd_example", displayName: "displayName_example", headers: "TODO", instructions: "instructions_example", tools: [Tool(annotations: Tool_annotations(destructiveHint: false, idempotentHint: false, openWorldHint: false, readOnlyHint: false, title: "title_example"), description: "description_example", icons: [Icon(mimeType: "mimeType_example", sizes: "sizes_example", src: "src_example")], inputSchema: "TODO", name: "name_example", outputSchema: "TODO", title: "title_example")], code: "code_example", dependencies: ["dependencies_example"])], instructions: "instructions_example", parameters: [RecipeParameter(_default: "_default_example", description: "description_example", inputType: RecipeParameterInputType(), key: "key_example", options: ["options_example"], requirement: RecipeParameterRequirement())], prompt: "prompt_example", response: _Response(jsonSchema: 123), retry: RetryConfig(checks: [SuccessCheck(command: "command_example", type: "type_example")], maxRetries: 123, onFailure: "onFailure_example", onFailureTimeoutSeconds: 123, timeoutSeconds: 123), settings: Settings(gooseModel: "gooseModel_example", gooseProvider: "gooseProvider_example", temperature: 123), subRecipes: [SubRecipe(description: "description_example", name: "name_example", path: "path_example", sequentialWhenRepeated: false, values: "TODO")], title: "title_example", version: "version_example")) // EncodeRecipeRequest | 

RecipeManagementAPI.encodeRecipe(encodeRecipeRequest: encodeRecipeRequest) { (response, error) in
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
 **encodeRecipeRequest** | [**EncodeRecipeRequest**](EncodeRecipeRequest.md) |  | 

### Return type

[**EncodeRecipeResponse**](EncodeRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listRecipes**
```swift
    open class func listRecipes(completion: @escaping (_ data: ListRecipeResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


RecipeManagementAPI.listRecipes() { (response, error) in
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

[**ListRecipeResponse**](ListRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **scanRecipe**
```swift
    open class func scanRecipe(scanRecipeRequest: ScanRecipeRequest, completion: @escaping (_ data: ScanRecipeResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let scanRecipeRequest = ScanRecipeRequest(recipe: Recipe(activities: ["activities_example"], author: Author(contact: "contact_example", metadata: "metadata_example"), context: ["context_example"], description: "description_example", extensions: [ExtensionConfig(availableTools: ["availableTools_example"], bundled: false, description: "description_example", envKeys: ["envKeys_example"], envs: "TODO", name: "name_example", timeout: 123, type: "type_example", uri: "uri_example", args: ["args_example"], cmd: "cmd_example", displayName: "displayName_example", headers: "TODO", instructions: "instructions_example", tools: [Tool(annotations: Tool_annotations(destructiveHint: false, idempotentHint: false, openWorldHint: false, readOnlyHint: false, title: "title_example"), description: "description_example", icons: [Icon(mimeType: "mimeType_example", sizes: "sizes_example", src: "src_example")], inputSchema: "TODO", name: "name_example", outputSchema: "TODO", title: "title_example")], code: "code_example", dependencies: ["dependencies_example"])], instructions: "instructions_example", parameters: [RecipeParameter(_default: "_default_example", description: "description_example", inputType: RecipeParameterInputType(), key: "key_example", options: ["options_example"], requirement: RecipeParameterRequirement())], prompt: "prompt_example", response: _Response(jsonSchema: 123), retry: RetryConfig(checks: [SuccessCheck(command: "command_example", type: "type_example")], maxRetries: 123, onFailure: "onFailure_example", onFailureTimeoutSeconds: 123, timeoutSeconds: 123), settings: Settings(gooseModel: "gooseModel_example", gooseProvider: "gooseProvider_example", temperature: 123), subRecipes: [SubRecipe(description: "description_example", name: "name_example", path: "path_example", sequentialWhenRepeated: false, values: "TODO")], title: "title_example", version: "version_example")) // ScanRecipeRequest | 

RecipeManagementAPI.scanRecipe(scanRecipeRequest: scanRecipeRequest) { (response, error) in
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
 **scanRecipeRequest** | [**ScanRecipeRequest**](ScanRecipeRequest.md) |  | 

### Return type

[**ScanRecipeResponse**](ScanRecipeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

