# Extension Manager Namespace Implementation

This document provides an overview of the namespace functionality implemented for the Goose extension manager.

## Overview

The namespace system provides access control for extensions, allowing you to organize and restrict access to resources and tools based on namespaces. This is useful for:

- **Security**: Restricting sensitive resources to specific extensions
- **Organization**: Grouping related functionality by namespace
- **Multi-tenancy**: Isolating different user groups or applications

## Architecture

The implementation consists of two main components:

### 1. NamespaceManager (`namespace_manager.rs`)

A standalone manager that handles namespace access control:

```rust
use goose::agents::{NamespaceManager, NamespaceResult};

let nm = NamespaceManager::new();

// Create namespaces
nm.create_namespace("production");
nm.create_namespace("development");

// Grant access
nm.grant_access("production", vec!["trusted_ext".to_string()]);
nm.grant_access("development", vec!["trusted_ext".to_string(), "dev_ext".to_string()]);

// Check access
assert!(nm.has_access("production", "trusted_ext"));
assert!(!nm.has_access("production", "dev_ext"));
```

### 2. ExtensionManagerWithNamespace (`extension_manager_namespace.rs`)

A wrapper around the existing ExtensionManager that adds namespace functionality:

```rust
use goose::agents::{ExtensionManager, ExtensionManagerWithNamespace};

let ext_manager = ExtensionManager::new();
let ns_manager = ExtensionManagerWithNamespace::new(ext_manager);

// Setup default namespaces
ns_manager.setup_default_namespaces().await;

// Create custom namespace
ns_manager.create_namespace("sensitive");
ns_manager.grant_namespace_access("sensitive", vec!["security_ext".to_string()]);
```

## Key Features

### Access Control

```rust
// Grant access to multiple extensions
ns_manager.grant_namespace_access("shared", vec![
    "ext1".to_string(),
    "ext2".to_string(),
]);

// Revoke access
ns_manager.revoke_namespace_access("shared", "ext1")?;

// Check access
if ns_manager.has_namespace_access("sensitive", "requesting_ext") {
    // Allow operation
}
```

### Resource Access with Namespace Control

```rust
use serde_json::json;

// Read resource with namespace check
let params = json!({
    "uri": "resource://sensitive/data",
    "namespace": "sensitive"
});

let result = ns_manager.read_resource_with_namespace_check(
    params,
    Some("security_ext")  // requesting extension
).await?;
```

### Namespace Management

```rust
// List all namespaces for an extension
let namespaces = ns_manager.list_extension_namespaces("ext1");

// List all extensions in a namespace
let extensions = ns_manager.list_namespace_extensions("production")?;

// Get comprehensive namespace information
let info = ns_manager.get_namespace_info().await;
```

### Extension Migration

```rust
// Move extension from one namespace to another
ns_manager.migrate_extension_to_namespace(
    "ext1",
    Some("development"),  // from namespace
    "production"          // to namespace
)?;
```

## Default Namespaces

The system automatically creates three default namespaces:

- **`public`**: All extensions have access by default
- **`shared`**: All extensions have access by default  
- **`private`**: No extensions have access by default

```rust
// Setup defaults (called automatically)
ns_manager.setup_default_namespaces().await;
```

## Error Handling

The system provides comprehensive error handling:

```rust
use goose::agents::NamespaceError;

match ns_manager.validate_namespace_access("tool", Some("restricted"), Some("ext1")) {
    Ok(()) => {
        // Access granted
    }
    Err(NamespaceError::AccessDenied(ext, ns)) => {
        println!("Extension {} denied access to namespace {}", ext, ns);
    }
    Err(NamespaceError::NamespaceNotFound(ns)) => {
        println!("Namespace {} not found", ns);
    }
    Err(NamespaceError::ExtensionNotFound(ext)) => {
        println!("Extension {} not registered", ext);
    }
}
```

## Integration with Existing Code

The `ExtensionManagerWithNamespace` implements `Deref` and `DerefMut` for the underlying `ExtensionManager`, so all existing functionality remains available:

```rust
// All ExtensionManager methods work directly
let tools = ns_manager.get_prefixed_tools(None).await?;
let extensions = ns_manager.list_extensions().await?;

// Plus new namespace functionality
ns_manager.create_namespace("custom");
```

## Testing

Comprehensive tests are included for both components:

```bash
# Run namespace-specific tests
cargo test --package goose namespace

# Run all tests
cargo test --package goose
```

## Thread Safety

Both `NamespaceManager` and `ExtensionManagerWithNamespace` are thread-safe and can be shared across async tasks using `Arc`:

```rust
use std::sync::Arc;

let ns_manager = Arc::new(ExtensionManagerWithNamespace::new(ext_manager));
let ns_manager_clone = Arc::clone(&ns_manager);

tokio::spawn(async move {
    ns_manager_clone.create_namespace("background_task");
});
```

## Performance Considerations

- Namespace operations use `DashMap` for concurrent access
- Access checks are O(1) average case
- Listing operations iterate through relevant collections
- Memory usage scales with number of namespaces and extensions

## Future Enhancements

Potential areas for extension:

1. **Hierarchical Namespaces**: Support for nested namespace structures
2. **Permission Levels**: Different access levels (read, write, admin)
3. **Namespace Inheritance**: Child namespaces inheriting parent permissions
4. **Audit Logging**: Track namespace access attempts
5. **Configuration Persistence**: Save namespace configurations to disk

## Example Usage Scenarios

### Multi-tenant Application

```rust
// Setup tenant isolation
ns_manager.create_namespace("tenant_a");
ns_manager.create_namespace("tenant_b");

ns_manager.grant_namespace_access("tenant_a", vec!["tenant_a_ext".to_string()]);
ns_manager.grant_namespace_access("tenant_b", vec!["tenant_b_ext".to_string()]);
```

### Development vs Production

```rust
// Separate environments
ns_manager.create_namespace("dev");
ns_manager.create_namespace("staging");
ns_manager.create_namespace("prod");

// Different access levels
ns_manager.grant_namespace_access("dev", vec!["dev_tools".to_string(), "debug_ext".to_string()]);
ns_manager.grant_namespace_access("prod", vec!["prod_tools".to_string()]);
```

### Security Boundaries

```rust
// Sensitive operations
ns_manager.create_namespace("admin");
ns_manager.create_namespace("finance");

ns_manager.grant_namespace_access("admin", vec!["admin_ext".to_string()]);
ns_manager.grant_namespace_access("finance", vec!["finance_ext".to_string(), "audit_ext".to_string()]);
```
