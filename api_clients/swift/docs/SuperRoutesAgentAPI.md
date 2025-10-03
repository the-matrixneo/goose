# SuperRoutesAgentAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addSubRecipes**](SuperRoutesAgentAPI.md#addsubrecipes) | **POST** /agent/add_sub_recipes | 
[**extendPrompt**](SuperRoutesAgentAPI.md#extendprompt) | **POST** /agent/prompt | 
[**getTools**](SuperRoutesAgentAPI.md#gettools) | **GET** /agent/tools | 
[**resumeAgent**](SuperRoutesAgentAPI.md#resumeagent) | **POST** /agent/resume | 
[**startAgent**](SuperRoutesAgentAPI.md#startagent) | **POST** /agent/start | 
[**updateAgentProvider**](SuperRoutesAgentAPI.md#updateagentprovider) | **POST** /agent/update_provider | 
[**updateRouterToolSelector**](SuperRoutesAgentAPI.md#updateroutertoolselector) | **POST** /agent/update_router_tool_selector | 
[**updateSessionConfig**](SuperRoutesAgentAPI.md#updatesessionconfig) | **POST** /agent/session_config | 


# **addSubRecipes**
```swift
    open class func addSubRecipes(addSubRecipesRequest: AddSubRecipesRequest, completion: @escaping (_ data: AddSubRecipesResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let addSubRecipesRequest = AddSubRecipesRequest(sessionId: "sessionId_example", subRecipes: [SubRecipe(description: "description_example", name: "name_example", path: "path_example", sequentialWhenRepeated: false, values: "TODO")]) // AddSubRecipesRequest | 

SuperRoutesAgentAPI.addSubRecipes(addSubRecipesRequest: addSubRecipesRequest) { (response, error) in
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
 **addSubRecipesRequest** | [**AddSubRecipesRequest**](AddSubRecipesRequest.md) |  | 

### Return type

[**AddSubRecipesResponse**](AddSubRecipesResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **extendPrompt**
```swift
    open class func extendPrompt(extendPromptRequest: ExtendPromptRequest, completion: @escaping (_ data: ExtendPromptResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let extendPromptRequest = ExtendPromptRequest(_extension: "_extension_example", sessionId: "sessionId_example") // ExtendPromptRequest | 

SuperRoutesAgentAPI.extendPrompt(extendPromptRequest: extendPromptRequest) { (response, error) in
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
 **extendPromptRequest** | [**ExtendPromptRequest**](ExtendPromptRequest.md) |  | 

### Return type

[**ExtendPromptResponse**](ExtendPromptResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTools**
```swift
    open class func getTools(sessionId: String, extensionName: String? = nil, completion: @escaping (_ data: [ToolInfo]?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let sessionId = "sessionId_example" // String | Required session ID to scope tools to a specific session
let extensionName = "extensionName_example" // String | Optional extension name to filter tools (optional)

SuperRoutesAgentAPI.getTools(sessionId: sessionId, extensionName: extensionName) { (response, error) in
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
 **sessionId** | **String** | Required session ID to scope tools to a specific session | 
 **extensionName** | **String** | Optional extension name to filter tools | [optional] 

### Return type

[**[ToolInfo]**](ToolInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **resumeAgent**
```swift
    open class func resumeAgent(resumeAgentRequest: ResumeAgentRequest, completion: @escaping (_ data: Session?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let resumeAgentRequest = ResumeAgentRequest(sessionId: "sessionId_example") // ResumeAgentRequest | 

SuperRoutesAgentAPI.resumeAgent(resumeAgentRequest: resumeAgentRequest) { (response, error) in
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
 **resumeAgentRequest** | [**ResumeAgentRequest**](ResumeAgentRequest.md) |  | 

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **startAgent**
```swift
    open class func startAgent(startAgentRequest: StartAgentRequest, completion: @escaping (_ data: Session?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let startAgentRequest = StartAgentRequest(recipe: Recipe(activities: ["activities_example"], author: Author(contact: "contact_example", metadata: "metadata_example"), context: ["context_example"], description: "description_example", extensions: [ExtensionConfig(availableTools: ["availableTools_example"], bundled: false, description: "description_example", envKeys: ["envKeys_example"], envs: "TODO", name: "name_example", timeout: 123, type: "type_example", uri: "uri_example", args: ["args_example"], cmd: "cmd_example", displayName: "displayName_example", headers: "TODO", instructions: "instructions_example", tools: [Tool(annotations: Tool_annotations(destructiveHint: false, idempotentHint: false, openWorldHint: false, readOnlyHint: false, title: "title_example"), description: "description_example", icons: [Icon(mimeType: "mimeType_example", sizes: "sizes_example", src: "src_example")], inputSchema: "TODO", name: "name_example", outputSchema: "TODO", title: "title_example")], code: "code_example", dependencies: ["dependencies_example"])], instructions: "instructions_example", parameters: [RecipeParameter(_default: "_default_example", description: "description_example", inputType: RecipeParameterInputType(), key: "key_example", options: ["options_example"], requirement: RecipeParameterRequirement())], prompt: "prompt_example", response: _Response(jsonSchema: 123), retry: RetryConfig(checks: [SuccessCheck(command: "command_example", type: "type_example")], maxRetries: 123, onFailure: "onFailure_example", onFailureTimeoutSeconds: 123, timeoutSeconds: 123), settings: Settings(gooseModel: "gooseModel_example", gooseProvider: "gooseProvider_example", temperature: 123), subRecipes: [SubRecipe(description: "description_example", name: "name_example", path: "path_example", sequentialWhenRepeated: false, values: "TODO")], title: "title_example", version: "version_example"), workingDir: "workingDir_example") // StartAgentRequest | 

SuperRoutesAgentAPI.startAgent(startAgentRequest: startAgentRequest) { (response, error) in
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
 **startAgentRequest** | [**StartAgentRequest**](StartAgentRequest.md) |  | 

### Return type

[**Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateAgentProvider**
```swift
    open class func updateAgentProvider(updateProviderRequest: UpdateProviderRequest, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let updateProviderRequest = UpdateProviderRequest(model: "model_example", provider: "provider_example", sessionId: "sessionId_example") // UpdateProviderRequest | 

SuperRoutesAgentAPI.updateAgentProvider(updateProviderRequest: updateProviderRequest) { (response, error) in
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
 **updateProviderRequest** | [**UpdateProviderRequest**](UpdateProviderRequest.md) |  | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateRouterToolSelector**
```swift
    open class func updateRouterToolSelector(updateRouterToolSelectorRequest: UpdateRouterToolSelectorRequest, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let updateRouterToolSelectorRequest = UpdateRouterToolSelectorRequest(sessionId: "sessionId_example") // UpdateRouterToolSelectorRequest | 

SuperRoutesAgentAPI.updateRouterToolSelector(updateRouterToolSelectorRequest: updateRouterToolSelectorRequest) { (response, error) in
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
 **updateRouterToolSelectorRequest** | [**UpdateRouterToolSelectorRequest**](UpdateRouterToolSelectorRequest.md) |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateSessionConfig**
```swift
    open class func updateSessionConfig(sessionConfigRequest: SessionConfigRequest, completion: @escaping (_ data: String?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let sessionConfigRequest = SessionConfigRequest(response: _Response(jsonSchema: 123), sessionId: "sessionId_example") // SessionConfigRequest | 

SuperRoutesAgentAPI.updateSessionConfig(sessionConfigRequest: sessionConfigRequest) { (response, error) in
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
 **sessionConfigRequest** | [**SessionConfigRequest**](SessionConfigRequest.md) |  | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

