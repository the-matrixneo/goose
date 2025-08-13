use anyhow::Result;
use chrono::{DateTime, Utc};
use indoc::indoc;
use mcp_core::handler::ToolError;
use mcp_core::protocol::ServerCapabilities;
use mcp_server::router::CapabilitiesBuilder;
use mcp_server::Router;
#[cfg(unix)]
use nix::{
    sys::signal::{kill as unix_kill, Signal},
    unistd::Pid,
};
use rmcp::model::{Content, JsonRpcMessage, Prompt, Resource, Role, Tool, ToolAnnotations};
use rmcp::object;
use serde_json::Value;
use std::fs::{self, File};
use std::future::Future;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Stdio;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
#[cfg(unix)]
use tokio::sync::mpsc;
use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::developer::DeveloperRouter;

const PM: &str = "npm";

// Global dev server state that persists across router instances
static GLOBAL_DEV_SERVER: OnceLock<Mutex<Option<Child>>> = OnceLock::new();


#[derive(Clone)]
pub struct BuildRouter {
    inner: DeveloperRouter,
    state: Arc<Mutex<BuildState>>, // shared process/log state
}

#[derive(Default)]
struct BuildState {
    child: Option<Child>,
    project_path: Option<PathBuf>,
    start_time: Option<DateTime<Utc>>,
    logs: Vec<String>,
    last_read_index: usize,
    port: Option<u16>,
}

impl Default for BuildRouter {
    fn default() -> Self {
        Self::new()
    }
}

fn get_global_dev_server() -> &'static Mutex<Option<Child>> {
    GLOBAL_DEV_SERVER.get_or_init(|| Mutex::new(None))
}

pub fn cleanup_global_dev_server() {
    eprintln!("[BuildRouter] Cleaning up global dev server");
    if let Some(global) = GLOBAL_DEV_SERVER.get() {
        if let Ok(mut child_opt) = global.lock() {
            if let Some(mut child) = child_opt.take() {
                if let Some(pid) = child.id() {
                    eprintln!("[BuildRouter] Killing dev server process group for PID: {}", pid);
                    
                    #[cfg(unix)]
                    {
                        // Kill the entire process group to ensure npm and its node children are killed
                        let pgid = -(pid as i32);
                        let _ = unix_kill(Pid::from_raw(pgid), Signal::SIGKILL);
                        let _ = unix_kill(Pid::from_raw(pid as i32), Signal::SIGKILL);
                    }
                    
                    #[cfg(not(unix))]
                    {
                        let _ = std::process::Command::new("taskkill")
                            .args(["/F", "/T", "/PID", &pid.to_string()])
                            .output();
                    }
                }
                let _ = child.start_kill();
            }
        }
    }
}

impl BuildRouter {
    pub fn new() -> Self {
        let this = Self {
            inner: DeveloperRouter::new(),
            state: Arc::new(Mutex::new(BuildState::default())),
        };

        // Best-effort: run in background, ignore errors in constructor
        let state = this.state.clone();
        if let Ok(cwd) = std::env::current_dir() {
            let pkg_json = cwd.join("package.json");
            if pkg_json.exists() {
                // Set the port synchronously before spawning async work
                let port = find_available_port(5173);
                {
                    let mut s = state.lock().unwrap();
                    s.project_path = Some(cwd.clone());
                    s.port = Some(port);
                }

                               tokio::spawn(async move {
                    let _ = install_deps_if_needed(&cwd).await;
                    let _ = start_dev_server(state.clone(), &cwd).await;
                });
            }
        }

        this
    }
}

impl Router for BuildRouter {
    fn name(&self) -> String {
        "build".to_string()
    }

    fn instructions(&self) -> String {
        indoc! {
            r#"
            When operating the build extension, your role is to help the user develop a web app.

            You are the code author, and should not expect the user to understand programming. You
            need to communicate as though the user is tech savvy but does not know anything about the
            underlying code. You can however respond with more technical detail if the user leans in or
            makes it clear they understand typescript.

            The project itself will always be based off of a react-router template and use tailwind.
            The project can be deployed to hosting infra with a reserved domain on block hosting.
            The project is mostly maintained on localhost, the deployed version is just a frozen snapshot.
            Expect the project to evolve over many sessions. You must maintain a PROJECT_SPEC.md file that
            enumerates every page and describes what it does. You should guide the user into communicating the
            details of that spec over time. You can fill in/extrapolate for them, but always follow the spec
            and represent any preferences the user sends you in the spec.
            "#
        }.to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        CapabilitiesBuilder::new()
            .with_tools(false)
            .with_prompts(false)
            .build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        let mut tools = self.inner.list_tools();

        let manage_server_tool = Tool::new(
            "manage_server",
            indoc! {
                r#"
                Manage the local web development project server and deployment lifecycle.

                Actions:
                - logs: Get logs emitted since the last time logs were checked (incremental)
                - restart: Install/update dependencies if needed and restart the dev server
                - build: Run the production build (npm run build)
                - deploy: Zip the build output and upload to the hosting endpoint
                - port: Get the port number the dev server is running on

                Notes:
                - Autostarts the dev server on MCP startup when a package.json is present in the current directory
                - Uses npm for all operations
                - Deploy expects a .goose-metadata.json with { "subdomain": "..." } in the project root
                "#
            },
            object!({
                "type": "object",
                "required": ["action"],
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["logs", "restart", "build", "deploy", "port"]
                    },
                    "message": {"type": "string", "description": "Optional deploy message"}
                }
            }),
        )
        .annotate(ToolAnnotations {
            title: Some("Manage development server".to_string()),
            read_only_hint: Some(false),
            destructive_hint: Some(false),
            idempotent_hint: Some(false),
            open_world_hint: Some(false),
        });

        tools.push(manage_server_tool);
        tools
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
        _notifier: mpsc::Sender<JsonRpcMessage>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<Content>, mcp_core::handler::ToolError>>
                + Send
                + 'static,
        >,
    > {
        let this = self.clone();
        let tool_name = tool_name.to_string();
        Box::pin(async move {
            match tool_name.as_str() {
                "manage_server" => this.manage_server(_notifier, arguments).await,
                _ => this.inner.call_tool(&tool_name, arguments, _notifier).await,
            }
        })
    }

    fn list_resources(&self) -> Vec<Resource> {
        self.inner.list_resources()
    }

    fn read_resource(
        &self,
        uri: &str,
    ) -> Pin<
        Box<dyn Future<Output = Result<String, mcp_core::handler::ResourceError>> + Send + 'static>,
    > {
        self.inner.read_resource(uri)
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        self.inner.list_prompts()
    }

    fn get_prompt(
        &self,
        prompt_name: &str,
    ) -> Pin<
        Box<dyn Future<Output = Result<String, mcp_core::handler::PromptError>> + Send + 'static>,
    > {
        self.inner.get_prompt(prompt_name)
    }
}

impl BuildRouter {
    async fn get_port(&self) -> Result<Vec<Content>, ToolError> {
        // Return the port we determined when starting the server
        let port = {
            let s = self.state.lock().unwrap();
            s.port.unwrap_or(5173) // Default to 5173 if not set
        };

        Ok(vec![Content::text(format!("{}", port))])
    }

    async fn manage_server(
        &self,
        _notifier: mpsc::Sender<JsonRpcMessage>,
        params: Value,
    ) -> Result<Vec<Content>, ToolError> {
        let action = params
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidParameters("Missing 'action'".to_string()))?;

        // No package manager override; we always use npm

        match action {
            "logs" => self.handle_logs_action().await,
            "restart" => self.handle_restart_action().await,
            "build" => self.handle_build_action().await,
            "deploy" => self.handle_deploy_action(&params).await,
            "port" => self.get_port().await,
            _ => Err(ToolError::InvalidParameters(format!(
                "Unsupported action: {}",
                action
            ))),
        }
    }

    async fn handle_logs_action(&self) -> Result<Vec<Content>, ToolError> {
        let (text, meta) = {
            let mut s = self.state.lock().unwrap();
            let start = s.last_read_index;
            let end = s.logs.len();
            let new_logs = s.logs[start..end].join("");
            s.last_read_index = end;
            
            // Check if global dev server is running
            let running = if let Some(global) = GLOBAL_DEV_SERVER.get() {
                if let Ok(child_opt) = global.lock() {
                    child_opt.is_some()
                } else {
                    false
                }
            } else {
                false
            };
            
            (
                new_logs.to_string(),
                format!("running: {running}, package_manager: {}", PM),
            )
        };
        Ok(vec![
            Content::text(meta).with_audience(vec![Role::Assistant]),
            Content::text(text)
        ])
    }

    async fn handle_restart_action(&self) -> Result<Vec<Content>, ToolError> {
        let project = {
            let mut s = self.state.lock().unwrap();
            let project = s
                .project_path
                .clone()
                .unwrap_or(std::env::current_dir().unwrap());

            s.logs.push(format!(
                "[{}] Restart requested for project: {}\n",
                Utc::now(),
                project.display()
            ));
            project
        };

        // Check for existing global process and terminate it
        let child_to_terminate = {
            let global = get_global_dev_server();
            if let Ok(mut child_opt) = global.lock() {
                if let Some(child) = child_opt.take() {
                    eprintln!("[BuildRouter] Terminating existing global dev server for restart");
                    Some(child)
                } else {
                    None
                }
            } else {
                None
            }
        };
        
        if let Some(child) = child_to_terminate {
            self.terminate_child_process(child).await;
        }

        // install deps if needed and start
        install_deps_if_needed(&project)
            .await
            .map_err(to_tool_err)?;
        start_dev_server(self.state.clone(), &project)
            .await
            .map_err(to_tool_err)?;

        Ok(vec![
            Content::text("Server restarted").with_audience(vec![Role::Assistant])
        ])    
    }

    async fn terminate_child_process(&self, mut child: Child) {
        // Try graceful shutdown first
        let pid_opt = child.id().map(|id| id as i32);
        {
            let mut s = self.state.lock().unwrap();
            if let Some(pid) = pid_opt {
                s.logs.push(format!(
                    "[{}] Sending SIGTERM to pid {} and its process group for graceful shutdown...\n",
                    Utc::now(),
                    pid
                ));
            } else {
                s.logs.push(format!(
                    "[{}] Child has no pid; attempting hard kill...\n",
                    Utc::now()
                ));
            }
        }

        #[cfg(unix)]
        {
            use tokio::time::{timeout, Duration};
            if let Some(pid) = pid_opt {
                // Try to kill the entire process group first
                let pgid = -(pid);
                let _ = unix_kill(Pid::from_raw(pgid), Signal::SIGTERM);
                
                // Also send SIGTERM to the specific process
                let _ = unix_kill(Pid::from_raw(pid), Signal::SIGTERM);
                
                match timeout(Duration::from_secs(3), child.wait()).await {
                    Ok(_status) => {
                        let mut s = self.state.lock().unwrap();
                        s.logs.push(format!(
                            "[{}] Process {} and its group exited cleanly after SIGTERM.\n",
                            Utc::now(),
                            pid
                        ));
                    }
                    Err(_elapsed) => {
                        {
                            let mut s = self.state.lock().unwrap();
                            s.logs.push(format!(
                                "[{}] Timeout waiting for pid {} to exit, sending SIGKILL to process group...\n",
                                Utc::now(),
                                pid
                            ));
                        }
                        // Kill the process group
                        let _ = unix_kill(Pid::from_raw(pgid), Signal::SIGKILL);
                        
                        // Also kill the specific process
                        let _ = child.kill().await; // SIGKILL
                        let _ = child.wait().await; // ensure it's fully terminated
                    }
                }
            } else {
                let _ = child.kill().await;
                let _ = child.wait().await;
            }
        }

        #[cfg(not(unix))]
        {
            // On non-Unix, no SIGTERM; fall back to hard kill
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
    }

    async fn handle_build_action(&self) -> Result<Vec<Content>, ToolError> {
        let project = {
            let s = self.state.lock().unwrap();
            s.project_path
                .clone()
                .unwrap_or(std::env::current_dir().unwrap())
        };
        let out = run_pm_command(&project, &pm_build_command()).await?;
        Ok(vec![Content::text(out).with_audience(vec![Role::User])])
    }

    async fn handle_deploy_action(&self, params: &Value) -> Result<Vec<Content>, ToolError> {
        let message = params
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Deployment from Build MCP");
        let project = {
            let s = self.state.lock().unwrap();
            s.project_path
                .clone()
                .unwrap_or(std::env::current_dir().unwrap())
        };
        let report = deploy_from_project(&project, message).await?;

        Ok(vec![Content::text(report)])
    }
}

fn find_available_port(start_port: u16) -> u16 {
    for port in start_port..=65535 {
        // Check both IPv4 and IPv6 to ensure the port is truly available
        // Try IPv4 first
        let ipv4_available = match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))) {
            Ok(listener) => {
                drop(listener);
                true
            }
            Err(_) => false,
        };

        // Try IPv6 - check ::1 (localhost)
        let ipv6_available =
            match TcpListener::bind(SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], port))) {
                Ok(listener) => {
                    drop(listener);
                    true
                }
                Err(_) => false,
            };

        // Also check 0.0.0.0 which would catch any wildcard bindings
        let wildcard_available = match TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))) {
            Ok(listener) => {
                drop(listener);
                true
            }
            Err(_) => false,
        };

        // Port is only available if all checks pass
        if ipv4_available && ipv6_available && wildcard_available {
            return port;
        }
    }
    start_port // fallback to start_port if no port is available
}

fn pm_install_command() -> Vec<&'static str> {
    vec!["npm", "install"]
}

fn pm_dev_command() -> Vec<&'static str> {
    vec!["npm", "run", "dev"]
}

fn pm_build_command() -> Vec<&'static str> {
    vec!["npm", "run", "build"]
}

async fn install_deps_if_needed(project_path: &Path) -> Result<()> {
    let _ = append_log(
        project_path,
        &format!("[{}] Installing dependencies with npm...\n", Utc::now()),
    );
    let out = run_pm_command(project_path, &pm_install_command()).await?;
    let _ = append_log(project_path, &format!("{out}\n"));

    Ok(())
}

async fn start_dev_server(state: Arc<Mutex<BuildState>>, project_path: &Path) -> Result<()> {
    // Check if global dev server is already running
    {
        let global = get_global_dev_server();
        if let Ok(child_opt) = global.lock() {
            if child_opt.is_some() {
                eprintln!("[BuildRouter] Dev server already running globally");
                return Ok(());
            }
        }
    }
    
    // Get the port that was already set synchronously, or find a new one if needed
    let port = {
        let mut s = state.lock().unwrap();

        // Use existing port if set, otherwise find a new one
        let port = s.port.unwrap_or_else(|| {
            let new_port = find_available_port(5173);
            s.port = Some(new_port);
            new_port
        });

        s.logs.push(format!(
            "[{}] Starting dev server in {} using npm on port {}...\n",
            Utc::now(),
            project_path.display(),
            port
        ));
        port
    };

    let mut cmd = Command::new(pm_dev_command()[0]);
    for arg in &pm_dev_command()[1..] {
        cmd.arg(arg);
    }
    cmd.arg("--");
    cmd.arg("--port");
    cmd.arg(port.to_string());
    
    // On Unix, create a new process group so we can kill all child processes
    #[cfg(unix)]
    cmd.process_group(0);
    
    let mut child = cmd
        .current_dir(project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| {
            eprintln!("[BuildRouter] Failed to spawn dev server: {}", e);
            e
        })?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    // Get the process ID before moving the child
    let child_pid = child.id();
    eprintln!("[BuildRouter] Dev server started with PID: {:?}", child_pid);
    
    eprintln!("[BuildRouter] Dev server running with PID: {:?}", child_pid);
    
    // Store in global state
    {
        let global = get_global_dev_server();
        if let Ok(mut child_opt) = global.lock() {
            *child_opt = Some(child);
            eprintln!("[BuildRouter] Stored dev server in global state");
        }
    }
    
    {
        let mut s = state.lock().unwrap();
        s.start_time = Some(Utc::now());
    }

    // Spawn log readers
    if let Some(stdout) = stdout {
        let state_clone = state.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let mut s = state_clone.lock().unwrap();
                s.logs.push(format!("[stdout] {line}\n"));
            }
            let mut s = state_clone.lock().unwrap();
            s.logs.push(format!("[{}] Dev server stdout stream closed\n", Utc::now()));
        });
    }
    if let Some(stderr) = stderr {
        let state_clone = state.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let mut s = state_clone.lock().unwrap();
                s.logs.push(format!("[stderr] {line}\n"));
                // Also log errors to stderr for debugging
                if line.contains("error") || line.contains("Error") || line.contains("ERROR") {
                    eprintln!("[BuildRouter] Dev server error: {}", line);
                }
            }
            // Log when stderr closes
            eprintln!("[BuildRouter] Dev server stderr closed");
            let mut s = state_clone.lock().unwrap();
            s.logs.push(format!("[{}] Dev server stderr stream closed\n", Utc::now()));
        });
    }
    
    // Monitor the process for early termination
    if let Some(pid) = child_pid {
        let state_clone = state.clone();
        tokio::spawn(async move {
            // Wait a bit then check if process is still alive
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            
            // Try to get a reference to the child to check its status
            let process_alive = {
                let s = state_clone.lock().unwrap();
                s.child.is_some()
            };
            
            if process_alive {
                eprintln!("[BuildRouter] Process {} still alive after 2 seconds", pid);
            } else {
                eprintln!("[BuildRouter] Process {} terminated within 2 seconds!", pid);
            }
            
            // Check again after 5 seconds
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            let process_still_alive = {
                let s = state_clone.lock().unwrap();
                s.child.is_some()
            };
            
            if !process_still_alive {
                eprintln!("[BuildRouter] Process {} confirmed dead after 5 seconds", pid);
            }
        });
    }
    
    Ok(())
}

async fn run_pm_command(project_path: &Path, cmd_vec: &[&str]) -> Result<String, ToolError> {
    let mut cmd = Command::new(cmd_vec[0]);
    for arg in &cmd_vec[1..] {
        cmd.arg(arg);
    }
    let output = cmd
        .current_dir(project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| ToolError::ExecutionError(format!("Failed to run {:?}: {}", cmd_vec, e)))?;

    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(ToolError::ExecutionError(format!(
            "Command {:?} failed with code {:?}:\n{}",
            cmd_vec,
            output.status.code(),
            text
        )));
    }

    Ok(text)
}

fn to_tool_err(e: anyhow::Error) -> ToolError {
    ToolError::ExecutionError(e.to_string())
}

fn append_log(project: &Path, _line: &str) -> Result<()> {
    let _ = project; // placeholder if we later want per-project files
                     // Currently just push to in-memory log; when called here we don't have access to state
    Ok(())
}

async fn deploy_from_project(project: &Path, message: &str) -> Result<String, ToolError> {
    // Read metadata
    let meta_path = project.join(".goose-metadata.json");
    if !meta_path.exists() {
        return Err(ToolError::ExecutionError(
            "No .goose-metadata.json found in project root".into(),
        ));
    }
    let mut meta_str = String::new();
    File::open(&meta_path)
        .map_err(|e| ToolError::ExecutionError(format!("Failed to open metadata: {}", e)))?
        .read_to_string(&mut meta_str)
        .map_err(|e| ToolError::ExecutionError(format!("Failed to read metadata: {}", e)))?;
    let meta: serde_json::Value = serde_json::from_str(&meta_str)
        .map_err(|e| ToolError::ExecutionError(format!("Invalid JSON in metadata: {}", e)))?;
    let subdomain = meta
        .get("subdomain")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::ExecutionError("'subdomain' missing in metadata".into()))?;

    // Ensure build exists
    let build_dir = project.join("build");
    if !build_dir.exists() {
        return Err(ToolError::ExecutionError(
            "Build directory not found. Run manage_server build first.".into(),
        ));
    }

    // Create archive in temp file
    let archive_path = project.join("archive.zip");
    if archive_path.exists() {
        let _ = fs::remove_file(&archive_path);
    }

    let file = File::create(&archive_path)
        .map_err(|e| ToolError::ExecutionError(format!("Failed to create archive: {}", e)))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for entry in WalkDir::new(&build_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let rel = path.strip_prefix(project).unwrap_or(path);
            zip.start_file(rel.to_string_lossy(), options)
                .map_err(|e| ToolError::ExecutionError(format!("Zip error: {}", e)))?;
            let mut f = File::open(path)
                .map_err(|e| ToolError::ExecutionError(format!("Zip read error: {}", e)))?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)
                .map_err(|e| ToolError::ExecutionError(format!("Zip read error: {}", e)))?;
            zip.write_all(&buf)
                .map_err(|e| ToolError::ExecutionError(format!("Zip write error: {}", e)))?;
        }
    }
    zip.finish()
        .map_err(|e| ToolError::ExecutionError(format!("Zip finalize error: {}", e)))?;

    // Upload
    let api = "https://goose-dev-sites.stage.sqprod.co/api/v1";
    let url = format!("{}/sites/{}/upload", api, subdomain);

    let client = reqwest::Client::new();
    // Build multipart form with in-memory bytes to avoid path-based file helper differences
    let mut archive_bytes = Vec::new();
    File::open(&archive_path)
        .map_err(|e| ToolError::ExecutionError(format!("Failed to re-open archive: {}", e)))?
        .read_to_end(&mut archive_bytes)
        .map_err(|e| ToolError::ExecutionError(format!("Failed to read archive: {}", e)))?;

    let file_part = reqwest::multipart::Part::bytes(archive_bytes)
        .file_name("archive.zip".to_string())
        .mime_str("application/zip")
        .map_err(|e| ToolError::ExecutionError(format!("Failed to build multipart part: {}", e)))?;

    let form = reqwest::multipart::Form::new()
        .text("message", message.to_string())
        .part("file", file_part);

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| ToolError::ExecutionError(format!("Upload failed: {}", e)))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| ToolError::ExecutionError(format!("Failed to read response: {}", e)))?;

    // Clean archive
    let _ = fs::remove_file(&archive_path);

    if !status.is_success() {
        return Err(ToolError::ExecutionError(format!(
            "Upload failed with status {}: {}",
            status, text
        )));
    }

    let site_url = format!("https://{}.vibeplatstage.squarecdn.com", subdomain);
    Ok(format!(
        "Successfully deployed. The website is available at: {}\nResponse: {}",
        site_url, text
    ))
}
