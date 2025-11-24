use std::process::Command;
pub fn set_power_profile(profile: &str) -> serde_json::Value {
    // profile: "balanced" or "high"
    let cmd = match profile {
        "high" => "powercfg -setactive SCHEME_MIN",
        "balanced" => "powercfg -setactive SCHEME_BALANCED",
        _ => "powercfg -getactivescheme"
    };
    let output = Command::new("cmd").arg("/C").arg(cmd).output();
    match output {
        Ok(o) => serde_json::json!({"cmd": cmd, "output": String::from_utf8_lossy(&o.stdout).to_string()}),
        Err(e) => serde_json::json!({"error": format!("{:?}", e)})
    }
}