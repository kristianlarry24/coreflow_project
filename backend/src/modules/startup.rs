use std::process::Command;
pub fn list_startup_items() -> serde_json::Value {
    // Using PowerShell to read common run keys
    let script = r#"Get-ItemProperty HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run, HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run | Format-List"#;
    let output = Command::new("powershell").arg("-NoProfile").arg("-NonInteractive").arg("-Command").arg(script).output();
    match output {
        Ok(o) => serde_json::json!({"output": String::from_utf8_lossy(&o.stdout).to_string()}),
        Err(e) => serde_json::json!({"error": format!("{:?}", e)})
    }
}