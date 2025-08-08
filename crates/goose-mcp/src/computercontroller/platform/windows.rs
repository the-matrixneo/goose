use super::SystemAutomation;
use std::path::PathBuf;
use std::process::Command;

pub struct WindowsAutomation;

impl SystemAutomation for WindowsAutomation {
    fn execute_system_script(&self, script: &str) -> std::io::Result<String> {
        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-NonInteractive")
            .arg("-Command")
            .arg(script)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn get_shell_command(&self) -> (&'static str, &'static str) {
        ("powershell", "-Command")
    }

    fn get_temp_path(&self) -> PathBuf {
        // Try unified config first, then TEMP env var, then Windows default
        goose::config::unified::get::<String>("system.temp_dir")
            .or_else(|_| std::env::var("TEMP"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(r"C:\Windows\Temp"))
    }
}
