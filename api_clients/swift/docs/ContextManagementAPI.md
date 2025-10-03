# ContextManagementAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**manageContext**](ContextManagementAPI.md#managecontext) | **POST** /context/manage | 


# **manageContext**
```swift
    open class func manageContext(contextManageRequest: ContextManageRequest, completion: @escaping (_ data: ContextManageResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let contextManageRequest = ContextManageRequest(manageAction: "manageAction_example", messages: [Message(content: [MessageContent(meta: "TODO", annotations: EmbeddedResource_annotations(audience: ["audience_example"], lastModified: Date(), priority: 123), text: "text_example", type: "type_example", data: "data_example", mimeType: "mimeType_example", id: "id_example", toolCall: 123, toolResult: 123, arguments: "TODO", prompt: "prompt_example", toolName: "toolName_example", signature: "signature_example", thinking: "thinking_example", msg: "msg_example")], created: 123, id: "id_example", metadata: MessageMetadata(agentVisible: false, userVisible: false), role: "role_example")], sessionId: "sessionId_example") // ContextManageRequest | 

ContextManagementAPI.manageContext(contextManageRequest: contextManageRequest) { (response, error) in
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
 **contextManageRequest** | [**ContextManageRequest**](ContextManageRequest.md) |  | 

### Return type

[**ContextManageResponse**](ContextManageResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

