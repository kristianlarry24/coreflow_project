use sysinfo::{System, SystemExt};

pub fn gather() -> serde_json::Value {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu = sys.global_processor_info().cpu_usage();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    serde_json::json!({
        "cpu_percent": cpu,
        "total_memory_kb": total_mem,
        "used_memory_kb": used_mem,
    })
}