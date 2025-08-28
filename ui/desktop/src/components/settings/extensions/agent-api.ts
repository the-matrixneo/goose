import { ExtensionConfig, ExtensionQuery } from '../../../api/types.gen';
import { addExtension, removeExtension } from '../../../api';
import { toastService, ToastServiceOptions } from '../../../toasts';
import { replaceWithShims } from './utils';

/**
 * Makes an API call to the extension endpoints using the generated API client
 */
export async function extensionApiCall(
  isAdd: boolean,
  payload: ExtensionConfig | string,
  options: ToastServiceOptions & { isDelete?: boolean } = {}
): Promise<any> {
  // Configure toast notifications
  toastService.configure(options);

  // Determine if we're activating, deactivating, or removing an extension
  const isRemoving = options.isDelete === true;

  const action = {
    type: isAdd ? 'activating' : isRemoving ? 'removing' : 'deactivating',
    verb: isAdd ? 'Activating' : isRemoving ? 'Removing' : 'Deactivating',
    pastTense: isAdd ? 'activated' : isRemoving ? 'removed' : 'deactivated',
    presentTense: isAdd ? 'activate' : isRemoving ? 'remove' : 'deactivate',
  };

  // for adding the payload is an extensionConfig, for removing payload is just the name
  const extensionName = isAdd ? (payload as ExtensionConfig).name : (payload as string);
  let toastId;

  // Step 1: Show loading toast (only for activation of stdio)
  if (isAdd && typeof payload === 'object' && payload.type === 'stdio') {
    toastId = toastService.loading({
      title: extensionName,
      msg: `${action.verb} ${extensionName} extension...`,
    });
  }

  try {
    // Step 2: Make the API call using the generated client
    let result;
    if (isAdd) {
      const extensionQuery: ExtensionQuery = {
        name: extensionName,
        config: payload as ExtensionConfig,
        enabled: true, // Extensions are enabled when added
      };
      result = await addExtension({
        body: extensionQuery,
      });
    } else {
      result = await removeExtension({
        path: { name: payload as string },
      });
    }

    // Step 3: Handle non-successful responses
    if (result.error) {
      const errorMsg = `API error: ${(result.error as any)?.message || 'Unknown error'}`;
      console.error(errorMsg);

      toastService.dismiss(toastId);
      toastService.error({
        title: extensionName,
        msg: `Failed to ${action.presentTense} extension`,
        traceback: errorMsg,
      });
      throw new Error(errorMsg);
    }

    // Step 4: Check for errors in the response data
    if (
      result.data &&
      typeof result.data === 'object' &&
      'error' in result.data &&
      (result.data as any).error
    ) {
      const errorMessage = `Error ${action.type} extension: ${(result.data as any).message || 'Unknown error'}`;
      toastService.dismiss(toastId);
      throw new Error(errorMessage);
    }

    // Step 5: Success - dismiss loading toast and return
    toastService.dismiss(toastId);
    toastService.success({
      title: extensionName,
      msg: `Successfully ${action.pastTense} extension`,
    });
    return result;
  } catch (error) {
    // Final catch-all error handler
    toastService.dismiss(toastId);
    const errorMessage = error instanceof Error ? error.message : String(error);
    const msg =
      errorMessage.length < 70 ? errorMessage : `Failed to ${action.presentTense} extension`;
    toastService.error({
      title: extensionName,
      msg: msg,
      traceback: errorMessage,
    });
    console.error(`Error in extensionApiCall for ${extensionName}:`, error);
    throw error;
  }
}

export async function addExtensionToAgent(
  extension: ExtensionConfig,
  options: ToastServiceOptions = {}
): Promise<any> {
  try {
    if (extension.type === 'stdio') {
      extension.cmd = await replaceWithShims(extension.cmd);
    }

    extension.name = sanitizeName(extension.name);

    return await extensionApiCall(true, extension, options);
  } catch (error) {
    // Check if this is a 428 error and make the message more descriptive
    if (error instanceof Error && error.message && error.message.includes('428')) {
      const enhancedError = new Error(
        'Failed to add extension. Goose Agent was still starting up. Please try again.'
      );
      console.error(`Failed to add extension ${extension.name} to agent: ${enhancedError.message}`);
      throw enhancedError;
    }
    throw error;
  }
}

/**
 * Remove an extension from the agent
 */
export async function removeFromAgent(
  name: string,
  options: ToastServiceOptions & { isDelete?: boolean } = {}
): Promise<any> {
  try {
    return await extensionApiCall(false, sanitizeName(name), options);
  } catch (error) {
    const action = options.isDelete ? 'remove' : 'deactivate';
    console.error(`Failed to ${action} extension ${name} from agent:`, error);
    throw error;
  }
}

export function sanitizeName(name: string) {
  return name.toLowerCase().replace(/-/g, '').replace(/_/g, '').replace(/\s/g, '');
}
