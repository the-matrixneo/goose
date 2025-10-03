# SuperRoutesReplyAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirmPermission**](SuperRoutesReplyAPI.md#confirmpermission) | **POST** /confirm | 


# **confirmPermission**
```swift
    open class func confirmPermission(permissionConfirmationRequest: PermissionConfirmationRequest, completion: @escaping (_ data: AnyCodable?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI

let permissionConfirmationRequest = PermissionConfirmationRequest(action: "action_example", id: "id_example", principalType: PrincipalType(), sessionId: "sessionId_example") // PermissionConfirmationRequest | 

SuperRoutesReplyAPI.confirmPermission(permissionConfirmationRequest: permissionConfirmationRequest) { (response, error) in
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
 **permissionConfirmationRequest** | [**PermissionConfirmationRequest**](PermissionConfirmationRequest.md) |  | 

### Return type

**AnyCodable**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

