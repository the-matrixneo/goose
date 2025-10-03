---
sidebar_position: 1
title: Session Management
sidebar_label: Session Management
---
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import { AppWindow, PanelLeft, FolderDot, Paperclip, Copy, Edit2, Trash2 } from 'lucide-react';


A session is a single, continuous interaction between you and Goose, providing a space to ask questions and prompt action. This guide covers how to manage the session lifecycle.

## Start Session 

:::info First-time setup
In your first session, Goose prompts you to [set up an LLM (Large Language Model) provider](/docs/getting-started/installation#set-llm-provider).
:::

<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
        When you open Goose, you'll see the session interface ready for use. Just type&mdash;[or speak](/docs/guides/sessions/in-session-actions#voice-dictation "Learn how to enable voice dictation")&mdash;your questions, requests, or instructions directly into the input field, and Goose will immediately get to work. 
        
        When you're ready to work on a new task, you can start a new session in the same directory or a different one. This directory is where Goose reads and writes files by default.

        <Tabs>
          <TabItem value="same-directory" label="Same Directory" default>

            To start a session in the same Goose window:
            1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar
            2. Click `Home` in the sidebar
            3. Send your first prompt from the chat box

            To start a session in a new Goose window:
            1. Click the <AppWindow className="inline" size={16} /> button in the top-left
            2. In the new Goose window, send your first prompt from the chat box

          </TabItem>
          <TabItem value="diff-directory" label="Different Directory">

            1. Click the <FolderDot className="inline" size={16} /> directory switcher at the bottom of the app
            2. Navigate to the new directory or create a new folder
            3. Click `Open` to open a new Goose window for the selected directory
            4. Send your first prompt from the chat box

          </TabItem>
        </Tabs>

        :::tip
        On macOS, you can drag and drop a folder onto the Goose icon in the dock to open a new session in that directory.
        :::

        You can also use keyboard shortcuts to start a new session or bring focus to open Goose windows.
        
        | Action | macOS | Windows/Linux |
        |--------|-------|---------------|
        | New Session in Current Directory | `Cmd+N`  | `Ctrl+N`  |
        | New Session in Different Directory  | `Cmd+O` | `Ctrl+O` |
        | Focus Goose Window | `Cmd+Option+Shift+G` | `Ctrl+Alt+Shift+G` |
        | Toggle Sidebar | `Cmd+B` | `Ctrl+B` |

    </TabItem>
    <TabItem value="cli" label="Goose CLI">
        From your terminal, navigate to the directory from which you'd like to start, and run the [session](/docs/guides/goose-cli-commands#session-options) command:
        ```sh
        goose session 
        ```

        If you want to interact with Goose in a web-based chat interface, start a session with the [`web`](/docs/guides/goose-cli-commands#web) command:
        ```sh
        goose web --open
        ```
    </TabItem>
</Tabs>

## Name Session
<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
        In the Desktop app, session tiles display auto-generated descriptions based on the context of your initial prompt.

        You can edit session descriptions after they're created:

        1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar
        2. Click `History` in the sidebar
        3. Hover over the session you'd like to rename
        4. Click the <Edit2 className="inline" size={16} /> button that appears on the session card
        5. In the "Edit Session Description" modal that opens:
           - Enter your new session description (up to 200 characters)
           - Press `Enter` to save or `Escape` to cancel
           - Or click the `Save` or `Cancel` buttons
        6. A success toast notification will confirm the change

        :::tip
        Session descriptions help you manage multiple Goose windows. When you're in the Goose chat interface, session descriptions appear in the `Window` menu and in the Dock (macOS) or taskbar (Windows) menu, making it easy to identify and switch between different Goose sessions.
        ::: 

    </TabItem>
    <TabItem value="cli" label="Goose CLI">
        By default, Goose names your session using the current timestamp in the format `YYYYMMDD_HHMMSS`. If you'd like to provide a specific name, this is where you'd do so. For example to name your session `react-migration`, you would run:

        ```
        goose session -n react-migration
        ```

        You'll know your session has started when your terminal looks similar to the following:

        ```
        starting session | provider: openai model: gpt-4o
        logging to ~/.local/share/goose/sessions/react-migration.json1
        ```
    </TabItem>
</Tabs>

## Exit Session
Note that sessions are automatically saved when you exit.
<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
    To exit a session, simply close the application.
    </TabItem>    
    <TabItem value="cli" label="Goose CLI">
        To exit a session, type `exit`. Alternatively, you exit the session by holding down `Ctrl+C`.

        Your session will be stored locally in `~/.local/share/goose/sessions`.
    </TabItem>
</Tabs>

## Search Sessions

Search allows you to find specific content within sessions or find specific sessions.

<Tabs groupId="interface">
  <TabItem value="ui" label="Goose Desktop" default>

    You can use keyboard shortcuts and search bar buttons to search sessions in Goose Desktop.

    | Action | macOS | Windows/Linux |
    |--------|-------|---------------|
    | Open Search | `Cmd+F`  | `Ctrl+F`  |
    | Next Match | `Cmd+G` or `↓` | `Ctrl+G` or `↓` |
    | Previous Match | `Shift+Cmd+G` or `↑` | `Shift+Ctrl+G` or `↑` |
    | Use Selection for Find | `Cmd+E` | n/a |
    | Toggle Case-Sensitivity | `Aa` | `Aa` |
    | Close Search | `Esc` or `X` | `Esc` or `X` |

    The following scenarios are supported:

    #### Search Within Current Session
    
    To find specific content within your current session:

    1. Use `Cmd+F` to open the search bar
    2. Enter your search term
    3. Use shortcuts and search bar buttons to navigate the results

    #### Search For Session By Name or Path
    
    To search all your sessions by name or working directory path:

    1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar
    2. Click `History` in the sidebar
    3. Use `Cmd+F` to open the search bar
    4. Enter your search term
    5. Use keyboard shortcuts and search bar buttons to navigate the results (`Cmd+E` not supported)

    This is a metadata-only search. It doesn't search conversation content. Note that searching by file name is supported (e.g. `20250727_130002.jsonl`), but this property isn't displayed in the UI.

    :::tip
    You can [rename sessions](#name-session) to give them descriptive names that you'll remember later.
    :::

    #### Search Within Historical Session
    
    To find specific content within a historical session:

    1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar
    2. Click `History` in the sidebar
    3. Click a specific session tile from the list to view its content
    4. Use `Cmd+F` to open the search bar
    5. Enter your search term
    6. Use keyboard shortcuts and search bar buttons to navigate the results

    :::info No Regex or operator support
    Using regular expressions or search operators in search text isn't supported.
    :::

  </TabItem>
  <TabItem value="cli" label="Goose CLI">

    Search functionality is provided by your terminal interface. Use the appropriate shortcut for your environment:

    | Terminal | Operating System | Shortcut |
    |----------|-----------------|-----------|
    | iTerm2 | macOS | `Cmd+F` |
    | Terminal.app | macOS | `Cmd+F` |
    | Windows Terminal | Windows | `Ctrl+F` |
    | Linux Terminal | Linux | `Ctrl+F` |

    :::info
    Your specific terminal emulator may use a different keyboard shortcut. Check your terminal's documentation or settings for the search command.
    :::

    The Goose CLI supports [listing session history](/docs/guides/goose-cli-commands#session-list-options) but doesn't provide search functionality. As a workaround, you can use your terminal's search capabilities (including regex support) to search for specific content within sessions or find specific sessions. 
    
    Examples for macOS:

    ```bash
    # Search session IDs (filenames)
    ls ~/.local/share/goose/sessions/ | grep "full or partial session id"

    # List sessions modified in last 7 days
    find ~/.local/share/goose/sessions/ -mtime -7 -name "*.jsonl"

    # Show first line (metadata) of each session file
    for f in ~/.local/share/goose/sessions/*.jsonl; do
        head -n1 "$f" | grep "your search term" && echo "Found in: $(basename "$f" .jsonl)"
    done

    # Find search term in session content
    rg "your search term" ~/.local/share/goose/sessions/

    # Search and show session IDs that contain search term
    for f in ~/.local/share/goose/sessions/*.jsonl; do
        if grep -q "your search term" "$f"; then
        echo "Found in session: $(basename "$f" .jsonl)"
        fi
    done
    ```

  </TabItem>
</Tabs>

## Resume Session

<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
    1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar
    2. Click `History` in the sidebar
    3. Click the session you'd like to resume. Goose provides [search features](#search-sessions) to help you find the session.
    4. Choose how to resume:
       - Click `Resume` to continue in the current window
       - Click `New Window` to open in a new window
    
    :::tip
    You can also quickly resume one of your three most recent sessions by clicking it in the `Recent chats` section on the `Home` page.
    :::

    </TabItem>
    <TabItem value="cli" label="Goose CLI">
        To resume your latest session, you can run the following command:

        ```
         goose session -r
        ```

        To resume a specific session, run the following command: 

        ```
        goose session -r --name <name>
        ```
        For example, to resume the session named `react-migration`, you would run:

        ```
        goose session -r --name react-migration
        ```

        :::tip
        While you can resume sessions using the commands above, we recommend creating new sessions for new tasks to reduce the chance of [doom spiraling](/docs/troubleshooting#stuck-in-a-loop-or-unresponsive).
        :::
    </TabItem>
</Tabs>

### Resume Session Across Interfaces

You can resume a CLI session in Desktop.

<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
    All saved sessions are listed in the Desktop app, even CLI sessions. To resume a CLI session within the Desktop:

    1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar
    2. Click `History` in the sidebar
    3. Click the session you'd like to resume
    4. Choose how to resume:
       - Click `Resume` to continue in the current window
       - Click `New Window` to open in a new window

    </TabItem>
    <TabItem value="cli" label="Goose CLI">
    Currently, you cannot resume a Desktop session within the CLI.
    </TabItem>
</Tabs>

### Resume Project-Based Sessions

<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
        Project-based sessions are only available through the CLI.
    </TabItem>
    <TabItem value="cli" label="Goose CLI">
        You can use the [`project`](/docs/guides/goose-cli-commands#project) and [`projects`](/docs/guides/goose-cli-commands#projects) commands to start or resume sessions from a project, which is a tracked working directory with session metadata. For a complete guide to using Projects, see [Managing Projects Guide](/docs/guides/managing-projects).
    </TabItem>
</Tabs>

## Delete Sessions

<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
        You can delete sessions directly from the Desktop app:

        1. Click the <PanelLeft className="inline" size={16} /> button in the top-left to open the sidebar
        2. Click `History` in the sidebar
        3. Find the session you want to delete
        4. Hover over the session card to reveal the action buttons
        5. Click the <Trash2 className="inline" size={16} /> button that appears
        6. Confirm the deletion in the modal that appears

        :::warning Permanent deletion
        Deleting a session from Goose Desktop will also delete it from the CLI. This action cannot be undone.
        :::

        The session will be immediately removed from your session history and the underlying session file will be deleted from your local storage.
    </TabItem>
    <TabItem value="cli" label="Goose CLI">
        You can remove sessions using CLI commands. For detailed instructions on session removal, see the [CLI Commands documentation](/docs/guides/goose-cli-commands#session-remove-options).
    </TabItem>
</Tabs>

## Export Sessions

Export sessions to Markdown to share with your team, create documentation, archive conversations, or review them in a readable format.

<Tabs groupId="interface">
    <TabItem value="ui" label="Goose Desktop" default>
        Session export is currently only available through the CLI.
    </TabItem>
    <TabItem value="cli" label="Goose CLI">
        Export sessions using the `export` subcommand:

        ```bash
        # Interactive export - prompts you to select a session
        goose session export
        ```
        
        For more details on export options, available flags, and output formats, see the [CLI commands documentation](/docs/guides/goose-cli-commands#session-export-options).
    </TabItem>
</Tabs>