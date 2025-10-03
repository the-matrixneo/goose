# goose-server Bash client

## Overview

This is a Bash client script for accessing goose-server service.

The script uses cURL underneath for making all REST calls.

## Usage

```shell
# Make sure the script has executable rights
$ chmod u+x goose-api

# Print the list of operations available on the service
$ ./goose-api -h

# Print the service description
$ ./goose-api --about

# Print detailed information about specific operation
$ ./goose-api <operationId> -h

# Make GET request
./goose-api --host http://<hostname>:<port> --accept xml <operationId> <queryParam1>=<value1> <header_key1>:<header_value2>

# Make GET request using arbitrary curl options (must be passed before <operationId>) to an SSL service using username:password
goose-api -k -sS --tlsv1.2 --host https://<hostname> -u <user>:<password> --accept xml <operationId> <queryParam1>=<value1> <header_key1>:<header_value2>

# Make POST request
$ echo '<body_content>' | goose-api --host <hostname> --content-type json <operationId> -

# Make POST request with simple JSON content, e.g.:
# {
#   "key1": "value1",
#   "key2": "value2",
#   "key3": 23
# }
$ echo '<body_content>' | goose-api --host <hostname> --content-type json <operationId> key1==value1 key2=value2 key3:=23 -

# Make POST request with form data
$ goose-api --host <hostname> <operationId> key1:=value1 key2:=value2 key3:=23

# Preview the cURL command without actually executing it
$ goose-api --host http://<hostname>:<port> --dry-run <operationid>

```

## Docker image

You can easily create a Docker image containing a preconfigured environment
for using the REST Bash client including working autocompletion and short
welcome message with basic instructions, using the generated Dockerfile:

```shell
docker build -t my-rest-client .
docker run -it my-rest-client
```

By default you will be logged into a Zsh environment which has much more
advanced auto completion, but you can switch to Bash, where basic autocompletion
is also available.

## Shell completion

### Bash

The generated bash-completion script can be either directly loaded to the current Bash session using:

```shell
source goose-api.bash-completion
```

Alternatively, the script can be copied to the `/etc/bash-completion.d` (or on OSX with Homebrew to `/usr/local/etc/bash-completion.d`):

```shell
sudo cp goose-api.bash-completion /etc/bash-completion.d/goose-api
```

#### OS X

On OSX you might need to install bash-completion using Homebrew:

```shell
brew install bash-completion
```

and add the following to the `~/.bashrc`:

```shell
if [ -f $(brew --prefix)/etc/bash_completion ]; then
  . $(brew --prefix)/etc/bash_completion
fi
```

### Zsh

In Zsh, the generated `_goose-api` Zsh completion file must be copied to one of the folders under `$FPATH` variable.

## Documentation for API Endpoints

All URIs are relative to **

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ContextManagementApi* | [**manageContext**](docs/ContextManagementApi.md#managecontext) | **POST** /context/manage | 
*RecipeManagementApi* | [**createRecipe**](docs/RecipeManagementApi.md#createrecipe) | **POST** /recipes/create | Create a Recipe configuration from the current session
*RecipeManagementApi* | [**decodeRecipe**](docs/RecipeManagementApi.md#decoderecipe) | **POST** /recipes/decode | 
*RecipeManagementApi* | [**deleteRecipe**](docs/RecipeManagementApi.md#deleterecipe) | **POST** /recipes/delete | 
*RecipeManagementApi* | [**encodeRecipe**](docs/RecipeManagementApi.md#encoderecipe) | **POST** /recipes/encode | 
*RecipeManagementApi* | [**listRecipes**](docs/RecipeManagementApi.md#listrecipes) | **GET** /recipes/list | 
*RecipeManagementApi* | [**scanRecipe**](docs/RecipeManagementApi.md#scanrecipe) | **POST** /recipes/scan | 
*ScheduleApi* | [**createSchedule**](docs/ScheduleApi.md#createschedule) | **POST** /schedule/create | 
*ScheduleApi* | [**deleteSchedule**](docs/ScheduleApi.md#deleteschedule) | **DELETE** /schedule/delete/{id} | 
*ScheduleApi* | [**inspectRunningJob**](docs/ScheduleApi.md#inspectrunningjob) | **GET** /schedule/{id}/inspect | 
*ScheduleApi* | [**killRunningJob**](docs/ScheduleApi.md#killrunningjob) | **POST** /schedule/{id}/kill | 
*ScheduleApi* | [**listSchedules**](docs/ScheduleApi.md#listschedules) | **GET** /schedule/list | 
*ScheduleApi* | [**pauseSchedule**](docs/ScheduleApi.md#pauseschedule) | **POST** /schedule/{id}/pause | 
*ScheduleApi* | [**runNowHandler**](docs/ScheduleApi.md#runnowhandler) | **POST** /schedule/{id}/run_now | 
*ScheduleApi* | [**sessionsHandler**](docs/ScheduleApi.md#sessionshandler) | **GET** /schedule/{id}/sessions | 
*ScheduleApi* | [**unpauseSchedule**](docs/ScheduleApi.md#unpauseschedule) | **POST** /schedule/{id}/unpause | 
*ScheduleApi* | [**updateSchedule**](docs/ScheduleApi.md#updateschedule) | **PUT** /schedule/{id} | 
*SessionManagementApi* | [**deleteSession**](docs/SessionManagementApi.md#deletesession) | **DELETE** /sessions/{session_id} | 
*SessionManagementApi* | [**getSession**](docs/SessionManagementApi.md#getsession) | **GET** /sessions/{session_id} | 
*SessionManagementApi* | [**getSessionInsights**](docs/SessionManagementApi.md#getsessioninsights) | **GET** /sessions/insights | 
*SessionManagementApi* | [**listSessions**](docs/SessionManagementApi.md#listsessions) | **GET** /sessions | 
*SessionManagementApi* | [**updateSessionDescription**](docs/SessionManagementApi.md#updatesessiondescription) | **PUT** /sessions/{session_id}/description | 
*SuperRoutesAgentApi* | [**addSubRecipes**](docs/SuperRoutesAgentApi.md#addsubrecipes) | **POST** /agent/add_sub_recipes | 
*SuperRoutesAgentApi* | [**extendPrompt**](docs/SuperRoutesAgentApi.md#extendprompt) | **POST** /agent/prompt | 
*SuperRoutesAgentApi* | [**getTools**](docs/SuperRoutesAgentApi.md#gettools) | **GET** /agent/tools | 
*SuperRoutesAgentApi* | [**resumeAgent**](docs/SuperRoutesAgentApi.md#resumeagent) | **POST** /agent/resume | 
*SuperRoutesAgentApi* | [**startAgent**](docs/SuperRoutesAgentApi.md#startagent) | **POST** /agent/start | 
*SuperRoutesAgentApi* | [**updateAgentProvider**](docs/SuperRoutesAgentApi.md#updateagentprovider) | **POST** /agent/update_provider | 
*SuperRoutesAgentApi* | [**updateRouterToolSelector**](docs/SuperRoutesAgentApi.md#updateroutertoolselector) | **POST** /agent/update_router_tool_selector | 
*SuperRoutesAgentApi* | [**updateSessionConfig**](docs/SuperRoutesAgentApi.md#updatesessionconfig) | **POST** /agent/session_config | 
*SuperRoutesConfigManagementApi* | [**addExtension**](docs/SuperRoutesConfigManagementApi.md#addextension) | **POST** /config/extensions | 
*SuperRoutesConfigManagementApi* | [**backupConfig**](docs/SuperRoutesConfigManagementApi.md#backupconfig) | **POST** /config/backup | 
*SuperRoutesConfigManagementApi* | [**createCustomProvider**](docs/SuperRoutesConfigManagementApi.md#createcustomprovider) | **POST** /config/custom-providers | 
*SuperRoutesConfigManagementApi* | [**getExtensions**](docs/SuperRoutesConfigManagementApi.md#getextensions) | **GET** /config/extensions | 
*SuperRoutesConfigManagementApi* | [**getProviderModels**](docs/SuperRoutesConfigManagementApi.md#getprovidermodels) | **GET** /config/providers/{name}/models | 
*SuperRoutesConfigManagementApi* | [**initConfig**](docs/SuperRoutesConfigManagementApi.md#initconfig) | **POST** /config/init | 
*SuperRoutesConfigManagementApi* | [**providers**](docs/SuperRoutesConfigManagementApi.md#providers) | **GET** /config/providers | 
*SuperRoutesConfigManagementApi* | [**readAllConfig**](docs/SuperRoutesConfigManagementApi.md#readallconfig) | **GET** /config | 
*SuperRoutesConfigManagementApi* | [**readConfig**](docs/SuperRoutesConfigManagementApi.md#readconfig) | **POST** /config/read | 
*SuperRoutesConfigManagementApi* | [**recoverConfig**](docs/SuperRoutesConfigManagementApi.md#recoverconfig) | **POST** /config/recover | 
*SuperRoutesConfigManagementApi* | [**removeConfig**](docs/SuperRoutesConfigManagementApi.md#removeconfig) | **POST** /config/remove | 
*SuperRoutesConfigManagementApi* | [**removeCustomProvider**](docs/SuperRoutesConfigManagementApi.md#removecustomprovider) | **DELETE** /config/custom-providers/{id} | 
*SuperRoutesConfigManagementApi* | [**removeExtension**](docs/SuperRoutesConfigManagementApi.md#removeextension) | **DELETE** /config/extensions/{name} | 
*SuperRoutesConfigManagementApi* | [**upsertConfig**](docs/SuperRoutesConfigManagementApi.md#upsertconfig) | **POST** /config/upsert | 
*SuperRoutesConfigManagementApi* | [**upsertPermissions**](docs/SuperRoutesConfigManagementApi.md#upsertpermissions) | **POST** /config/permissions | 
*SuperRoutesConfigManagementApi* | [**validateConfig**](docs/SuperRoutesConfigManagementApi.md#validateconfig) | **GET** /config/validate | 
*SuperRoutesHealthApi* | [**status**](docs/SuperRoutesHealthApi.md#status) | **GET** /status | 
*SuperRoutesReplyApi* | [**confirmPermission**](docs/SuperRoutesReplyApi.md#confirmpermission) | **POST** /confirm | 
*SuperRoutesSetupApi* | [**startOpenrouterSetup**](docs/SuperRoutesSetupApi.md#startopenroutersetup) | **POST** /handle_openrouter | 
*SuperRoutesSetupApi* | [**startTetrateSetup**](docs/SuperRoutesSetupApi.md#starttetratesetup) | **POST** /handle_tetrate | 


## Documentation For Models

 - [AddSubRecipesRequest](docs/AddSubRecipesRequest.md)
 - [AddSubRecipesResponse](docs/AddSubRecipesResponse.md)
 - [Annotations](docs/Annotations.md)
 - [Author](docs/Author.md)
 - [AuthorRequest](docs/AuthorRequest.md)
 - [ConfigKey](docs/ConfigKey.md)
 - [ConfigKeyQuery](docs/ConfigKeyQuery.md)
 - [ConfigResponse](docs/ConfigResponse.md)
 - [Content](docs/Content.md)
 - [ContextLengthExceeded](docs/ContextLengthExceeded.md)
 - [ContextManageRequest](docs/ContextManageRequest.md)
 - [ContextManageResponse](docs/ContextManageResponse.md)
 - [CreateCustomProviderRequest](docs/CreateCustomProviderRequest.md)
 - [CreateRecipeRequest](docs/CreateRecipeRequest.md)
 - [CreateRecipeResponse](docs/CreateRecipeResponse.md)
 - [CreateScheduleRequest](docs/CreateScheduleRequest.md)
 - [DecodeRecipeRequest](docs/DecodeRecipeRequest.md)
 - [DecodeRecipeResponse](docs/DecodeRecipeResponse.md)
 - [DeleteRecipeRequest](docs/DeleteRecipeRequest.md)
 - [EmbeddedResource](docs/EmbeddedResource.md)
 - [EmbeddedResourceAnnotations](docs/EmbeddedResourceAnnotations.md)
 - [EncodeRecipeRequest](docs/EncodeRecipeRequest.md)
 - [EncodeRecipeResponse](docs/EncodeRecipeResponse.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ExtendPromptRequest](docs/ExtendPromptRequest.md)
 - [ExtendPromptResponse](docs/ExtendPromptResponse.md)
 - [ExtensionConfig](docs/ExtensionConfig.md)
 - [ExtensionConfigOneOf](docs/ExtensionConfigOneOf.md)
 - [ExtensionConfigOneOf1](docs/ExtensionConfigOneOf1.md)
 - [ExtensionConfigOneOf2](docs/ExtensionConfigOneOf2.md)
 - [ExtensionConfigOneOf3](docs/ExtensionConfigOneOf3.md)
 - [ExtensionConfigOneOf4](docs/ExtensionConfigOneOf4.md)
 - [ExtensionConfigOneOf5](docs/ExtensionConfigOneOf5.md)
 - [ExtensionEntry](docs/ExtensionEntry.md)
 - [ExtensionQuery](docs/ExtensionQuery.md)
 - [ExtensionResponse](docs/ExtensionResponse.md)
 - [FrontendToolRequest](docs/FrontendToolRequest.md)
 - [GetToolsQuery](docs/GetToolsQuery.md)
 - [Icon](docs/Icon.md)
 - [ImageContent](docs/ImageContent.md)
 - [InspectJobResponse](docs/InspectJobResponse.md)
 - [KillJobResponse](docs/KillJobResponse.md)
 - [ListRecipeResponse](docs/ListRecipeResponse.md)
 - [ListSchedulesResponse](docs/ListSchedulesResponse.md)
 - [Message](docs/Message.md)
 - [MessageContent](docs/MessageContent.md)
 - [MessageContentOneOf](docs/MessageContentOneOf.md)
 - [MessageContentOneOf1](docs/MessageContentOneOf1.md)
 - [MessageContentOneOf2](docs/MessageContentOneOf2.md)
 - [MessageContentOneOf3](docs/MessageContentOneOf3.md)
 - [MessageContentOneOf4](docs/MessageContentOneOf4.md)
 - [MessageContentOneOf5](docs/MessageContentOneOf5.md)
 - [MessageContentOneOf6](docs/MessageContentOneOf6.md)
 - [MessageContentOneOf7](docs/MessageContentOneOf7.md)
 - [MessageContentOneOf8](docs/MessageContentOneOf8.md)
 - [MessageContentOneOf9](docs/MessageContentOneOf9.md)
 - [MessageMetadata](docs/MessageMetadata.md)
 - [ModelInfo](docs/ModelInfo.md)
 - [PermissionConfirmationRequest](docs/PermissionConfirmationRequest.md)
 - [PermissionLevel](docs/PermissionLevel.md)
 - [PrincipalType](docs/PrincipalType.md)
 - [ProviderDetails](docs/ProviderDetails.md)
 - [ProviderMetadata](docs/ProviderMetadata.md)
 - [ProvidersResponse](docs/ProvidersResponse.md)
 - [RawAudioContent](docs/RawAudioContent.md)
 - [RawEmbeddedResource](docs/RawEmbeddedResource.md)
 - [RawImageContent](docs/RawImageContent.md)
 - [RawResource](docs/RawResource.md)
 - [RawTextContent](docs/RawTextContent.md)
 - [Recipe](docs/Recipe.md)
 - [RecipeManifestResponse](docs/RecipeManifestResponse.md)
 - [RecipeParameter](docs/RecipeParameter.md)
 - [RecipeParameterInputType](docs/RecipeParameterInputType.md)
 - [RecipeParameterRequirement](docs/RecipeParameterRequirement.md)
 - [RedactedThinkingContent](docs/RedactedThinkingContent.md)
 - [ResourceContents](docs/ResourceContents.md)
 - [ResourceContentsAnyOf](docs/ResourceContentsAnyOf.md)
 - [ResourceContentsAnyOf1](docs/ResourceContentsAnyOf1.md)
 - [Response](docs/Response.md)
 - [ResumeAgentRequest](docs/ResumeAgentRequest.md)
 - [RetryConfig](docs/RetryConfig.md)
 - [RunNowResponse](docs/RunNowResponse.md)
 - [ScanRecipeRequest](docs/ScanRecipeRequest.md)
 - [ScanRecipeResponse](docs/ScanRecipeResponse.md)
 - [ScheduledJob](docs/ScheduledJob.md)
 - [Session](docs/Session.md)
 - [SessionConfigRequest](docs/SessionConfigRequest.md)
 - [SessionDisplayInfo](docs/SessionDisplayInfo.md)
 - [SessionInsights](docs/SessionInsights.md)
 - [SessionListResponse](docs/SessionListResponse.md)
 - [SessionsQuery](docs/SessionsQuery.md)
 - [Settings](docs/Settings.md)
 - [SetupResponse](docs/SetupResponse.md)
 - [StartAgentRequest](docs/StartAgentRequest.md)
 - [SubRecipe](docs/SubRecipe.md)
 - [SuccessCheck](docs/SuccessCheck.md)
 - [SummarizationRequested](docs/SummarizationRequested.md)
 - [TextContent](docs/TextContent.md)
 - [ThinkingContent](docs/ThinkingContent.md)
 - [Tool](docs/Tool.md)
 - [ToolAnnotations](docs/ToolAnnotations.md)
 - [ToolConfirmationRequest](docs/ToolConfirmationRequest.md)
 - [ToolInfo](docs/ToolInfo.md)
 - [ToolPermission](docs/ToolPermission.md)
 - [ToolRequest](docs/ToolRequest.md)
 - [ToolResponse](docs/ToolResponse.md)
 - [UpdateProviderRequest](docs/UpdateProviderRequest.md)
 - [UpdateRouterToolSelectorRequest](docs/UpdateRouterToolSelectorRequest.md)
 - [UpdateScheduleRequest](docs/UpdateScheduleRequest.md)
 - [UpdateSessionDescriptionRequest](docs/UpdateSessionDescriptionRequest.md)
 - [UpsertConfigQuery](docs/UpsertConfigQuery.md)
 - [UpsertPermissionsQuery](docs/UpsertPermissionsQuery.md)


## Documentation For Authorization

 All endpoints do not require authorization.

