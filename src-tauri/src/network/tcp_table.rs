use std::ffi::c_void;
use std::net::Ipv4Addr;
use windows_sys::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, NO_ERROR};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, MIB_TCPROW_OWNER_PID, MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_ALL,
};
use windows_sys::Win32::Networking::WinSock::AF_INET;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TcpConnection {
    pub pid: u32,
    pub local_addr: String,
    pub local_port: u16,
    pub remote_addr: String,
    pub remote_port: u16,
    pub state: String,
    pub protocol: &'static str,
}

pub fn get_tcp_connections() -> Vec<TcpConnection> {
    let mut connections = Vec::new();
    let mut size: u32 = 0;

    unsafe {
        let ret = GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if ret != ERROR_INSUFFICIENT_BUFFER as u32 && ret != NO_ERROR {
            return connections;
        }

        let mut buffer: Vec<u8> = vec![0; size as usize];

        if GetExtendedTcpTable(
            buffer.as_mut_ptr() as *mut c_void,
            &mut size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        ) == NO_ERROR
        {
            let table = &*(buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID);
            let rows = std::slice::from_raw_parts(
                table.table.as_ptr() as *const MIB_TCPROW_OWNER_PID,
                table.dwNumEntries as usize,
            );

            for row in rows {
                let local_ip = Ipv4Addr::from(u32::from_be(row.dwLocalAddr));
                let local_port = u16::from_be(row.dwLocalPort as u16);
                let remote_ip = Ipv4Addr::from(u32::from_be(row.dwRemoteAddr));
                let remote_port = u16::from_be(row.dwRemotePort as u16);

                let state_str = match row.dwState {
                    1 => "CLOSED",
                    2 => "LISTEN",
                    3 => "SYN_SENT",
                    4 => "SYN_RCVD",
                    5 => "ESTABLISHED",
                    6 => "FIN_WAIT1",
                    7 => "FIN_WAIT2",
                    8 => "CLOSE_WAIT",
                    9 => "CLOSING",
                    10 => "LAST_ACK",
                    11 => "TIME_WAIT",
                    _ => "UNKNOWN",
                };

                connections.push(TcpConnection {
                    pid: row.dwOwningPid,
                    local_addr: local_ip.to_string(),
                    local_port,
                    remote_addr: remote_ip.to_string(),
                    remote_port,
                    state: state_str.to_string(),
                    protocol: "TCP",
                });
            }
        }
    }

    connections
}
