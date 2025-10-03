# SuperRoutesSetupAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**startOpenrouterSetup**](SuperRoutesSetupAPI.md#startopenroutersetup) | **POST** /handle_openrouter | 
[**startTetrateSetup**](SuperRoutesSetupAPI.md#starttetratesetup) | **POST** /handle_tetrate | 


# **startOpenrouterSetup**
```swift
    open class func startOpenrouterSetup(completion: @escaping (_ data: SetupResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesSetupAPI.startOpenrouterSetup() { (response, error) in
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

[**SetupResponse**](SetupResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **startTetrateSetup**
```swift
    open class func startTetrateSetup(completion: @escaping (_ data: SetupResponse?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import GooseAPI


SuperRoutesSetupAPI.startTetrateSetup() { (response, error) in
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

[**SetupResponse**](SetupResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

