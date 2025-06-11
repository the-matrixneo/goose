---
title: Prevent Goose from Accessing Files
sidebar_label: Using Gooseignore
sidebar_position: 14
---


`.gooseignore` is a text file that defines patterns for files and directories that Goose will not access. This means Goose cannot read, modify, delete, or run shell commands on these files when using the Developer extension's tools.

:::info Developer extension only
The .gooseignore feature currently only affects tools in the [Developer](/docs/tutorials/developer-mcp) extension. Other extensions are not restricted by these rules.
:::

This guide will show you how to use `.gooseignore` files to prevent Goose from changing specific files and directories.

## Creating your `.gooseignore` file

Goose supports two types of `.gooseignore` files:
- **Global ignore file** - Create a `.gooseignore` file in `~/.config/goose`. These restrictions will apply to all your sessions with Goose, regardless of directory.
- **Local ignore file** - Create a `.gooseignore` file at the root of the directory you'd like it applied to. These restrictions will only apply when working in a specific directory.

:::tip
You can use both global and local `.gooseignore` files simultaneously. When both exist, Goose will combine the restrictions from both files to determine which paths are restricted.
:::

## Example `.gooseignore` file

In your `.gooseignore` file, you can write patterns to match files you want Goose to ignore. Here are some common patterns:

```plaintext
# Ignore specific files by name
settings.json         # Ignore only the file named "settings.json"

# Ignore files by extension
*.pdf                # Ignore all PDF files
*.config             # Ignore all files ending in .config

# Ignore directories and their contents
backup/              # Ignore everything in the "backup" directory
downloads/           # Ignore everything in the "downloads" directory

# Ignore all files with this name in any directory
**/credentials.json  # Ignore all files named "credentials.json" in any directory

# Complex patterns
*.log                # Ignore all .log files
!error.log           # Except for error.log file
```

## How Goose Chooses Which Files to Ignore

Goose uses a priority system to determine which files should be ignored. Here's how it works, from highest to lowest priority:

1. **Global `.gooseignore`** (`~/.config/goose/.gooseignore`)
   - Always applied first
   - Affects all projects on your machine
   - Perfect for personal preferences or system-wide restrictions

2. **Local `.gooseignore`** (in your project directory)
   - Project-specific rules
   - Overrides `.gitignore` completely
   - Use this when you want different rules than Git

3. **Local `.gitignore`** (in your project directory)
   - Only used if no local `.gooseignore` exists
   - Convenient for existing projects
   - Goose will respect the same files Git ignores

4. **Default Patterns**
   - Used when no other ignore files are found
   - Protects common sensitive files

### Examples of How Priority Works

1. **Most Common Setup** (using existing Git project):
   ```
   Project/
   ├── .gitignore        ← Goose uses this automatically
   ├── src/
   └── ...
   ```

2. **Custom Project Rules** (overriding Git's ignore rules):
   ```
   Project/
   ├── .gitignore        ← Ignored by Goose
   ├── .gooseignore      ← Takes precedence
   ├── src/
   └── ...
   ```

3. **Maximum Protection** (using both global and local rules):
   ```
   ~/.config/goose/
   └── .gooseignore      ← Applied first (global rules)
   
   Project/
   ├── .gooseignore      ← Applied second (local rules)
   ├── .gitignore        ← Ignored by Goose
   ├── src/
   └── ...
   ```

:::tip
If you're working on an existing project that uses Git, you probably don't need to create a `.gooseignore` file - Goose will automatically use your `.gitignore` rules!
:::

## Default patterns

By default, if you haven't created any `.gooseignore` files **and no `.gitignore` file exists**, Goose will not modify files matching these patterns:

```plaintext
**/.env
**/.env.*
**/secrets.*
```

These default patterns only apply when neither `.gooseignore` nor `.gitignore` files are found in your project.

## Common use cases

Here are some typical scenarios where `.gooseignore` is helpful:

- **Generated Files**: Prevent Goose from modifying auto-generated code or build outputs
- **Third-Party Code**: Keep Goose from changing external libraries or dependencies
- **Important Configurations**: Protect critical configuration files from accidental modifications
- **Version Control**: Prevent changes to version control files like `.git` directory
- **Existing Projects**: Most projects already have `.gitignore` files that work automatically as ignore patterns for Goose
- **Custom Restrictions**: Create `.gooseignore` when you need different patterns than your `.gitignore` (e.g., allowing Goose to read files that Git ignores)