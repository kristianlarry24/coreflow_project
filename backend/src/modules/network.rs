use std::process::Command;
pub fn ping_google() -> serde_json::Value {
    // Run ping -n 4 8.8.8.8 on Windows
    let output = Command::new("ping").arg("-n").arg("4").arg("8.8.8.8").output();
    match output {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            serde_json::json!({"cmd":"ping -n 4 8.8.8.8", "output": s})
        }
        Err(e) => serde_json::json!({"error": format!("{:?}", e)})
    }
}