use sysinfo::{System, SystemExt, ProcessExt};
pub fn list_processes() -> serde_json::Value {
    let mut sys = System::new_all();
    sys.refresh_all();
    let procs = sys.processes().iter().map(|(pid, proc_)| {
        serde_json::json!({
            "pid": pid.as_u32(),
            "name": proc_.name(),
            "cpu": proc_.cpu_usage(),
            "memory_kb": proc_.memory()
        })
    }).collect::<Vec<_>>();
    serde_json::json!({"processes": procs})
}