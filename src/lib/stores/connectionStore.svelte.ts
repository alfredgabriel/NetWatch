import { listen } from '@tauri-apps/api/event';
import type { EnrichedConnection, ProcessGroup } from '$lib/types/network';

class ConnectionStore {
  connections = $state<EnrichedConnection[]>([]);
  isPaused = $state<boolean>(false);
  seenPids = $state<Set<number>>(new Set());
  newPidsCount = $state<number>(0);
  searchQuery = $state<string>('');
  filter = $state<'all' | 'active' | 'new' | 'unresolved'>('all');

  // Filtered & grouped process list using Svelte 5 $derived.by
  processGroups = $derived.by<ProcessGroup[]>(() => {
    const groupsMap = new Map<number, ProcessGroup>();

    for (const conn of this.connections) {
      if (this.searchQuery.trim()) {
        const q = this.searchQuery.toLowerCase();
        const matchesName = conn.process_name.toLowerCase().includes(q);
        const matchesIp = conn.remote.toLowerCase().includes(q);
        const matchesDomain = conn.domain.toLowerCase().includes(q);
        const matchesPid = conn.pid.toString().includes(q);

        if (!matchesName && !matchesIp && !matchesDomain && !matchesPid) {
          continue;
        }
      }

      if (this.filter === 'active' && conn.state !== 'ESTABLISHED' && conn.protocol !== 'UDP') {
        continue;
      }
      if (this.filter === 'unresolved' && conn.domain !== conn.remote) {
        continue;
      }

      if (!groupsMap.has(conn.pid)) {
        groupsMap.set(conn.pid, {
          pid: conn.pid,
          process_name: conn.process_name,
          process_path: conn.process_path,
          connections: [],
          active_count: 0,
          total_remote_domains: []
        });
      }

      const group = groupsMap.get(conn.pid)!;
      group.connections.push(conn);
      if (conn.state === 'ESTABLISHED' || conn.protocol === 'UDP') {
        group.active_count += 1;
      }
      if (conn.domain && conn.domain !== '*' && !group.total_remote_domains.includes(conn.domain)) {
        group.total_remote_domains.push(conn.domain);
      }
    }

    return Array.from(groupsMap.values()).sort((a, b) => b.connections.length - a.connections.length);
  });

  async initListener() {
    try {
      await listen<EnrichedConnection[]>('network-update', (event) => {
        if (!this.isPaused) {
          const newConns = event.payload;
          
          let newlyDiscovered = 0;
          const currentSeen = new Set(this.seenPids);
          for (const conn of newConns) {
            if (!currentSeen.has(conn.pid)) {
              currentSeen.add(conn.pid);
              newlyDiscovered += 1;
            }
          }

          if (newlyDiscovered > 0 && this.seenPids.size > 0) {
            this.newPidsCount += newlyDiscovered;
          }

          this.seenPids = currentSeen;
          this.connections = newConns;
        }
      });
    } catch (err) {
      console.warn('Tauri event listener non-desktop fallback active', err);
    }
  }

  togglePause() {
    this.isPaused = !this.isPaused;
  }
}

export const connectionStore = new ConnectionStore();
