// Example demonstrating how to use the RMCP feature flag
// This shows how future migration phases can conditionally use RMCP vs legacy code

use mcp_core::config;

fn main() {
    println!("RMCP Migration Feature Flag Demo");
    println!("================================");
    
    // Check the current state of the feature flag
    println!("USE_RMCP constant: {}", config::USE_RMCP);
    println!("use_rmcp(): {}", config::use_rmcp());
    println!("use_legacy(): {}", config::use_legacy());
    
    // Example of how this would be used in practice during migration
    if config::use_rmcp() {
        println!("\n🚀 Using RMCP (official Rust SDK) implementation");
        // Future code would use RMCP types and functions here
        // e.g., rmcp::Service, rmcp::model::Content, etc.
    } else {
        println!("\n🔧 Using legacy internal MCP implementation");
        // Current code continues to use internal types
        // e.g., mcp_core::Content, mcp_core::Tool, etc.
    }
    
    println!("\nTo enable RMCP, change USE_RMCP to true in mcp-core/src/lib.rs");
}
