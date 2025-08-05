use goose::config::{Config, ENV_REGISTRY};
use std::time::Instant;

/// Demo showing the new environment variable registry system
/// 
/// This example demonstrates:
/// 1. Environment variables are loaded once at process start
/// 2. Accessing config values uses the pre-loaded registry (in production)
/// 3. Performance benefits of eliminating just-in-time env access
fn main() {
    println!("🐚 Goose Environment Registry Demo");
    println!("==================================");
    
    // Show environment variable diagnostics
    let diagnostics = ENV_REGISTRY.get_diagnostics();
    println!("\n📊 Environment Variable Statistics:");
    println!("  Total environment variables: {}", diagnostics.total_env_vars);
    println!("  Known Goose variables found: {}", diagnostics.known_found.len());
    println!("  Known Goose variables missing: {}", diagnostics.known_missing.len());
    println!("  Unknown GOOSE_* variables: {}", diagnostics.unknown_goose_vars.len());
    
    if !diagnostics.known_found.is_empty() {
        println!("\n✅ Found Goose environment variables:");
        for var in &diagnostics.known_found {
            println!("  - {}", var);
        }
    }
    
    if !diagnostics.unknown_goose_vars.is_empty() {
        println!("\n❓ Unknown GOOSE_* variables (consider adding to registry):");
        for var in &diagnostics.unknown_goose_vars {
            println!("  - {}", var);
        }
    }
    
    // Demonstrate config access
    println!("\n🔧 Configuration Access Demo:");
    let config = Config::global();
    
    // Try to get some common environment variables
    let test_vars = [
        ("HOME", false),
        ("USER", false), 
        ("GOOSE_MODEL", false),
        ("OPENAI_API_KEY", true),
        ("ANTHROPIC_API_KEY", true),
    ];
    
    for (var_name, is_secret) in &test_vars {
        match if *is_secret {
            config.get_secret::<String>(var_name)
        } else {
            config.get_param::<String>(var_name)
        } {
            Ok(value) => {
                if *is_secret {
                    println!("  🔐 {} = [REDACTED] (secret)", var_name);
                } else {
                    println!("  📝 {} = {}", var_name, value);
                }
            }
            Err(_) => {
                println!("  ❌ {} = <not set>", var_name);
            }
        }
    }
    
    // Performance comparison (rough benchmark)
    println!("\n⚡ Performance Comparison:");
    
    // Test the new registry-based approach
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = ENV_REGISTRY.get_raw("HOME");
        let _ = ENV_REGISTRY.get_raw("USER");
        let _ = ENV_REGISTRY.get_raw("GOOSE_MODEL");
    }
    let registry_time = start.elapsed();
    
    // Test the old just-in-time approach
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = std::env::var("HOME");
        let _ = std::env::var("USER");
        let _ = std::env::var("GOOSE_MODEL");
    }
    let jit_time = start.elapsed();
    
    println!("  Registry-based access: {:?} (for 3000 lookups)", registry_time);
    println!("  Just-in-time access:   {:?} (for 3000 lookups)", jit_time);
    
    if jit_time > registry_time {
        let speedup = jit_time.as_nanos() as f64 / registry_time.as_nanos() as f64;
        println!("  🚀 Registry is {:.1}x faster!", speedup);
    }
    
    // Show categorized environment variables
    println!("\n📂 Environment Variables by Category:");
    
    use goose::config::EnvCategory;
    let categories = [
        (EnvCategory::Provider, "Provider Configuration"),
        (EnvCategory::Core, "Core Configuration"),
        (EnvCategory::Interface, "Interface Configuration"),
        (EnvCategory::Debug, "Debug Configuration"),
        (EnvCategory::System, "System Environment"),
        (EnvCategory::Scheduler, "Scheduler Configuration"),
        (EnvCategory::Extension, "Extension/MCP Configuration"),
        (EnvCategory::Tracing, "Tracing/Observability"),
    ];
    
    for (category, name) in &categories {
        let vars = ENV_REGISTRY.get_by_category(category.clone());
        if !vars.is_empty() {
            println!("  {} ({} variables):", name, vars.len());
            for (key, _) in vars.iter().take(3) {
                println!("    - {}", key);
            }
            if vars.len() > 3 {
                println!("    ... and {} more", vars.len() - 3);
            }
        }
    }
    
    // Show structured discovery results
    use goose::config::{discover_provider_env_vars, discover_extension_env_vars};
    
    println!("\n🔍 Structured Discovery Results:");
    let discovered_providers = discover_provider_env_vars();
    println!("  Discovered {} provider config keys from structured metadata", discovered_providers.len());
    
    // Show a sample of discovered keys
    println!("  Sample discovered provider keys:");
    for spec in discovered_providers.iter().take(8) {
        let secret_indicator = if spec.is_secret { " 🔐" } else { " 📝" };
        println!("    -{}{} - {}", secret_indicator, spec.name, spec.description);
    }

    let discovered_extensions = discover_extension_env_vars();
    if !discovered_extensions.is_empty() {
        println!("  Discovered {} extension env_keys from YAML config", discovered_extensions.len());
        for key in discovered_extensions.iter().take(3) {
            println!("    - 🧩 {}", key);
        }
    } else {
        println!("  No extension env_keys found in current YAML config");
    }

    println!("\n✨ Environment registry loaded {} variables at process start", 
             diagnostics.total_env_vars);
    println!("   No more just-in-time environment variable access!");
    println!("🎯 Now using structured discovery instead of pattern matching!");
}