use std::ffi::c_void;
use std::net::Ipv4Addr;
use windows_sys::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, NO_ERROR};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetExtendedUdpTable, MIB_UDPROW_OWNER_PID, MIB_UDPTABLE_OWNER_PID, UDP_TABLE_OWNER_PID,
};
use windows_sys::Win32::Networking::WinSock::AF_INET;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UdpConnection {
    pub pid: u32,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub state: String,
    pub protocol: &'static str,
}

pub fn get_udp_connections() -> Vec<UdpConnection> {
    let mut connections = Vec::new();
    let mut size: u32 = 0;

    unsafe {
        let ret = GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut size,
            0,
            AF_INET as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if ret != ERROR_INSUFFICIENT_BUFFER as u32 && ret != NO_ERROR {
            return connections;
        }

        let mut buffer: Vec<u8> = vec![0; size as usize];

        if GetExtendedUdpTable(
            buffer.as_mut_ptr() as *mut c_void,
            &mut size,
            0,
            AF_INET as u32,
            UDP_TABLE_OWNER_PID,
            0,
        ) == NO_ERROR
        {
            let table = &*(buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID);
            let rows = std::slice::from_raw_parts(
                table.table.as_ptr() as *const MIB_UDPROW_OWNER_PID,
                table.dwNumEntries as usize,
            );

            for row in rows {
                let local_ip = Ipv4Addr::from(u32::from_be(row.dwLocalAddr));
                let local_port = u16::from_be(row.dwLocalPort as u16);

                connections.push(UdpConnection {
                    pid: row.dwOwningPid,
                    local_addr: local_ip.to_string(),
                    local_port,
                    remote_addr: "*".to_string(),
                    remote_port: 0,
                    state: "BOUND".to_string(),
                    protocol: "UDP",
                });
            }
        }
    }

    connections
}
