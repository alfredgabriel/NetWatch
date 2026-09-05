<script lang="ts">
  import { onMount } from 'svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import TopBar from '$lib/components/TopBar.svelte';
  import MetricsBar from '$lib/components/MetricsBar.svelte';
  import ConnectionsTable from '$lib/components/ConnectionsTable.svelte';
  import StatisticsView from '$lib/components/StatisticsView.svelte';
  import HistoryView from '$lib/components/HistoryView.svelte';
  import SettingsView from '$lib/components/SettingsView.svelte';
  import { connectionStore } from '$lib/stores/connectionStore.svelte';
  import { bandwidthStore } from '$lib/stores/bandwidthStore.svelte';

  // Navigation & View states
  let activeView = $state<string>('live');
  let viewMode = $state<'flat' | 'grouped'>('flat');
  let activeFilter = $state<string>('all');

  onMount(() => {
    connectionStore.initListener();
    const interval = setInterval(() => {
      bandwidthStore.updateSpeed(connectionStore.connections.length);
    }, 1500);
    return () => clearInterval(interval);
  });

  // Filtered connections list according to search and selected filter pill
  let filteredConnections = $derived.by(() => {
    const q = connectionStore.searchQuery.trim().toLowerCase();

    return connectionStore.connections.filter((conn) => {
      // Search text match
      if (q) {
        const matchesName = conn.process_name.toLowerCase().includes(q);
        const matchesIp = conn.remote.toLowerCase().includes(q);
        const matchesLocal = conn.local.toLowerCase().includes(q);
        const matchesDomain = conn.domain.toLowerCase().includes(q);
        const matchesPid = conn.pid.toString().includes(q);
        if (!matchesName && !matchesIp && !matchesLocal && !matchesDomain && !matchesPid) {
          return false;
        }
      }

      // Filter chips
      if (activeFilter === 'tcp') return conn.protocol === 'TCP';
      if (activeFilter === 'udp') return conn.protocol === 'UDP';
      if (activeFilter === 'established') return conn.state === 'ESTABLISHED';
      if (activeFilter === 'listen') return conn.state === 'LISTEN';

      return true;
    });
  });

  // Filtered process groups for grouped view
  let filteredProcessGroups = $derived.by(() => {
    const groupsMap = new Map<number, typeof connectionStore.processGroups[0]>();

    for (const conn of filteredConnections) {
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

  let establishedCount = $derived.by(() => {
    return connectionStore.connections.filter(c => c.state === 'ESTABLISHED').length;
  });
</script>

<div class="flex h-screen w-screen overflow-hidden" style="background:var(--bg-main);">
  <!-- Sidebar -->
  <Sidebar
    {activeView}
    onViewSelect={(v) => (activeView = v)}
  />

  <!-- Main Content Viewport -->
  <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
    <!-- Top search, filter and action bar -->
    <TopBar
      searchQuery={connectionStore.searchQuery}
      isPaused={connectionStore.isPaused}
      {activeFilter}
      {viewMode}
      onSearchChange={(q) => (connectionStore.searchQuery = q)}
      onTogglePause={() => connectionStore.togglePause()}
      onFilterChange={(f) => (activeFilter = f)}
      onViewModeChange={(m) => (viewMode = m)}
    />

    <!-- Metrics Summary Header -->
    <MetricsBar
      activeSocketsCount={connectionStore.connections.length}
      activeProcessesCount={connectionStore.processGroups.length}
      {establishedCount}
    />

    <!-- Active View Switcher -->
    {#if activeView === 'live'}
      <ConnectionsTable
        connections={filteredConnections}
        processGroups={filteredProcessGroups}
        {viewMode}
      />
    {:else if activeView === 'statistics'}
      <StatisticsView
        connections={connectionStore.connections}
        processGroups={connectionStore.processGroups}
      />
    {:else if activeView === 'history'}
      <HistoryView
        connections={connectionStore.connections}
      />
    {:else if activeView === 'settings'}
      <SettingsView />
    {/if}
  </div>
</div>
