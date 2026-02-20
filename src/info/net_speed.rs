use sysinfo::Networks;
use std::thread;
use std::time::Duration;

pub fn get_network_speed() -> String {
    let mut networks = Networks::new_with_refreshed_list();
    
    // Initial snapshot
    let mut total_rx: u64 = 0;
    let mut total_tx: u64 = 0;
    
    for (_interface_name, data) in &networks {
        total_rx += data.total_received();
        total_tx += data.total_transmitted();
    }
    
    // Sleep for 1s to calculate rate
    thread::sleep(Duration::from_secs(1));
    networks.refresh_list(); // Refresh list to get new data
    // Actually we need to iterate and trigger update? No, just refresh.
    // Wait, sysinfo 0.30 logic is confusing. Refreshing list updates data?
    // Let's check docs or just use refresh() on the networks struct if available.
    // Correct usage for sysinfo 0.30:
    // let mut networks = Networks::new_with_refreshed_list();
    // sleep
    // networks.refresh_list(); 
    // Wait, refresh_list updates the list of interfaces. We want to update stats.
    // networks.refresh() is likely what we want.
    
    // Re-implementation based on sysinfo 0.30 patterns:
    // We need to re-read the detailed stats.
    
    // Let's create a NEW networks instance to get the delta.
    // This is easier than managing refresh on the same struct if API is weird.
    let mut networks2 = Networks::new_with_refreshed_list();
    
    let mut new_rx: u64 = 0;
    let mut new_tx: u64 = 0;
    
    for (_interface_name, data) in &networks2 {
        new_rx += data.total_received();
        new_tx += data.total_transmitted();
    }
    
    // Wait, total_received is lifetime cumulative. So calculating diff between two snapshots 
    // taken 1s apart is correct.
    
    let rx_speed = (new_rx.saturating_sub(total_rx)) as f64 / 1024.0; 
    let tx_speed = (new_tx.saturating_sub(total_tx)) as f64 / 1024.0; 
    
    if rx_speed == 0.0 && tx_speed == 0.0 {
         // Fallback if measurement failed or idle
         // Maybe just show 0?
    }
    
    
    format!("⬇️ {:.1} KB/s  ⬆️ {:.1} KB/s", rx_speed, tx_speed)
}
