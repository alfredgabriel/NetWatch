use dashmap::DashMap;
use std::net::IpAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct DnsResolver {
    cache: Arc<DashMap<String, String>>,
}

impl DnsResolver {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    pub fn get_cached_or_resolve(&self, ip_str: String) -> String {
        if let Some(domain) = self.cache.get(&ip_str) {
            return domain.value().clone();
        }

        let cache_clone = self.cache.clone();
        let ip_clone = ip_str.clone();

        tauri::async_runtime::spawn(async move {
            if let Ok(ip) = ip_clone.parse::<IpAddr>() {
                if let Ok(name) = dns_lookup::lookup_addr(&ip) {
                    cache_clone.insert(ip_clone, name);
                    return;
                }
            }
            cache_clone.insert(ip_clone.clone(), ip_clone);
        });

        ip_str
    }
}
