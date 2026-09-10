pub mod dns;
pub mod network;
pub mod process;

use std::time::Duration;
use tauri::{Emitter, Manager};

#[derive(serde::Serialize, Clone, Debug)]
pub struct EnrichedConnection {
    pub pid: u32,
    pub process_name: String,
    pub process_path: String,
    pub local: String,
    pub remote: String,
    pub domain: String,
    pub state: String,
    pub protocol: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(icon) = app.default_window_icon() {
                for (_, window) in app.webview_windows() {
                    let _ = window.set_icon(icon.clone());
                }
            }
            let app_handle = app.handle().clone();
            let dns_resolver = dns::resolver::DnsResolver::new();

            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(300)).await;

                    let mut enriched = Vec::new();

                    let tcp_conns = network::tcp_table::get_tcp_connections();
                    for conn in tcp_conns {
                        let path = process::proc_info::get_process_path(conn.pid);
                        let path_buf = path.unwrap_or_default();
                        let proc_name = path_buf
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "System".into());

                        let domain = dns_resolver.get_cached_or_resolve(conn.remote_addr.clone());

                        enriched.push(EnrichedConnection {
                            pid: conn.pid,
                            process_name: proc_name,
                            process_path: path_buf.to_string_lossy().to_string(),
                            local: format!("{}:{}", conn.local_addr, conn.local_port),
                            remote: format!("{}:{}", conn.remote_addr, conn.remote_port),
                            domain,
                            state: conn.state,
                            protocol: conn.protocol.to_string(),
                        });
                    }

                    let udp_conns = network::udp_table::get_udp_connections();
                    for conn in udp_conns {
                        let path = process::proc_info::get_process_path(conn.pid);
                        let path_buf = path.unwrap_or_default();
                        let proc_name = path_buf
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "System".into());

                        enriched.push(EnrichedConnection {
                            pid: conn.pid,
                            process_name: proc_name,
                            process_path: path_buf.to_string_lossy().to_string(),
                            local: format!("{}:{}", conn.local_addr, conn.local_port),
                            remote: "*:*".to_string(),
                            domain: "*".to_string(),
                            state: conn.state,
                            protocol: conn.protocol.to_string(),
                        });
                    }

                    let _ = app_handle.emit("network-update", enriched);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
