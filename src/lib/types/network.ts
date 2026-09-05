export interface EnrichedConnection {
  pid: number;
  process_name: string;
  process_path: string;
  local: string;
  remote: string;
  domain: string;
  state: string;
  protocol: 'TCP' | 'UDP';
  first_seen?: number;
}

export interface ProcessGroup {
  process_name: string;
  process_path: string;
  pid: number;
  connections: EnrichedConnection[];
  active_count: number;
  total_remote_domains: string[];
}
