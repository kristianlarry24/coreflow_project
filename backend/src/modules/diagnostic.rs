pub fn run_corecheck() -> serde_json::Value {
    // Lightweight composite diagnostic: gather sysinfo, processes summary and disk info (placeholder)
    let sys = super::sysinfo_mod::gather();
    let procs = super::process::list_processes();
    serde_json::json!({"sysinfo": sys, "processes_summary": procs, "recommendation": "Review top CPU consumers and check disk health."})
}