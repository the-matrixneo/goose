use anyhow::{anyhow, Result};
use goose_mcp::{
    AutoVisualiserRouter, ComputerControllerRouter, DeveloperServer, MemoryServer, TutorialServer,
};
use rmcp::{transport::stdio, ServiceExt};

pub async fn run(name: &str) -> Result<()> {
    crate::logging::setup_logging(Some(&format!("mcp-{name}")))?;

    if name == "googledrive" || name == "google_drive" {
        return Err(anyhow!(
            "the built-in Google Drive extension has been removed"
        ));
    }

    tracing::info!("Starting MCP server");

    if name == "autovisualiser" {
        let service = AutoVisualiserRouter::new()
            .serve(stdio())
            .await
            .inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;

        service.waiting().await?;
        return Ok(());
    }

    if name == "computercontroller" {
        let service = ComputerControllerRouter::new()
            .serve(stdio())
            .await
            .inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;

        service.waiting().await?;
        return Ok(());
    }

    if name == "developer" {
        let service = DeveloperServer::new()
            .serve(stdio())
            .await
            .inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;

        service.waiting().await?;
        return Ok(());
    }

    if name == "memory" {
        let service = MemoryServer::new().serve(stdio()).await.inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

        service.waiting().await?;
        return Ok(());
    }

    if name == "tutorial" {
        let service = TutorialServer::new()
            .serve(stdio())
            .await
            .inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;

        service.waiting().await?;
        return Ok(());
    }

    return Ok(());
}
