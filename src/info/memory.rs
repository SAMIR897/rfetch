pub fn get_memory() -> String {
    let sys = sysinfo::System::new_all();
    let total = sys.total_memory() / 1024 / 1024; // Convert bytes to MiB
    let used = sys.used_memory() / 1024 / 1024;
    format!("{}MiB / {}MiB", used, total)
}
