<script lang="ts">
  import type { EnrichedConnection, ProcessGroup } from '$lib/types/network';
  import { t } from 'svelte-i18n';
  import {
    ShieldOff, ArrowUpDown, ChevronRight, ChevronDown, Copy,
    Check, X, ExternalLink, Globe, Cpu, Network
  } from 'lucide-svelte';

  let {
    connections = [],
    processGroups = [],
    viewMode = 'flat',
  }: {
    connections?: EnrichedConnection[];
    processGroups?: ProcessGroup[];
    viewMode?: 'flat' | 'grouped';
  } = $props();

  // Selected connection for detail modal/drawer
  let selectedConn = $state<EnrichedConnection | null>(null);
  let copiedField = $state<string | null>(null);

  // Group expansion state
  let expandedPids = $state<Set<number>>(new Set());

  function toggleGroup(pid: number) {
    const next = new Set(expandedPids);
    if (next.has(pid)) {
      next.delete(pid);
    } else {
      next.add(pid);
    }
    expandedPids = next;
  }

  // Sorting state
  type SortField = 'process' | 'remote' | 'proto' | 'state' | 'pid';
  let sortField = $state<SortField>('process');
  let sortAsc = $state<boolean>(true);

  function handleSort(field: SortField) {
    if (sortField === field) {
      sortAsc = !sortAsc;
    } else {
      sortField = field;
      sortAsc = true;
    }
  }

  let sortedConnections = $derived.by(() => {
    return [...connections].sort((a, b) => {
      let res = 0;
      if (sortField === 'process') res = a.process_name.localeCompare(b.process_name);
      else if (sortField === 'remote') res = (a.domain !== '*' ? a.domain : a.remote).localeCompare(b.domain !== '*' ? b.domain : b.remote);
      else if (sortField === 'proto') res = a.protocol.localeCompare(b.protocol);
      else if (sortField === 'state') res = a.state.localeCompare(b.state);
      else if (sortField === 'pid') res = a.pid - b.pid;
      return sortAsc ? res : -res;
    });
  });

  function statusPill(state: string) {
    if (state === 'ESTABLISHED' || state === 'BOUND') return 'pill-established';
    if (state === 'LISTEN') return 'pill-listen';
    return 'pill-close';
  }

  function protoPill(proto: string) {
    return proto === 'TCP' ? 'pill-tcp' : 'pill-udp';
  }

  function remotePort(remote: string) {
    const parts = remote.split(':');
    return parts.length > 1 ? parts[parts.length - 1] : '—';
  }

  function avatarColor(name: string) {
    const colors = ['#3B82F6', '#8B5CF6', '#EC4899', '#F59E0B', '#10B981', '#06B6D4'];
    let h = 0;
    for (let i = 0; i < name.length; i++) h = (h * 31 + name.charCodeAt(i)) % colors.length;
    return colors[h];
  }

  async function copyText(text: string, id: string) {
    try {
      await navigator.clipboard.writeText(text);
      copiedField = id;
      setTimeout(() => {
        if (copiedField === id) copiedField = null;
      }, 1800);
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="relative flex flex-col flex-1 overflow-hidden select-none">
  <!-- Table Header with Sort Buttons -->
  <div class="flex items-center px-4 py-2 text-[11px] font-semibold uppercase tracking-wider shrink-0 border-b select-none"
    style="color:var(--text-muted); border-color:var(--border); background:var(--bg-row-b);">
    
    <!-- Process -->
    <button
      onclick={() => handleSort('process')}
      class="flex items-center gap-1 hover:text-slate-200 transition-colors"
      style="flex: 2.8; padding-left: 36px;"
    >
      <span>{$t('table.process')}</span>
      <ArrowUpDown class="w-3 h-3 text-slate-500" />
    </button>

    <!-- Remote Destination -->
    <button
      onclick={() => handleSort('remote')}
      class="flex items-center gap-1 hover:text-slate-200 transition-colors"
      style="flex: 3.2;"
    >
      <span>{$t('table.remote_destination')}</span>
      <ArrowUpDown class="w-3 h-3 text-slate-500" />
    </button>

    <!-- Port / Proto -->
    <button
      onclick={() => handleSort('proto')}
      class="flex items-center justify-center gap-1 hover:text-slate-200 transition-colors"
      style="flex: 1.2;"
    >
      <span>{$t('table.port_proto')}</span>
      <ArrowUpDown class="w-3 h-3 text-slate-500" />
    </button>

    <!-- State -->
    <button
      onclick={() => handleSort('state')}
      class="flex items-center justify-center gap-1 hover:text-slate-200 transition-colors"
      style="flex: 1.2;"
    >
      <span>{$t('table.state')}</span>
      <ArrowUpDown class="w-3 h-3 text-slate-500" />
    </button>

    <!-- Local Address -->
    <span style="flex: 1.8; text-align: right; padding-right: 12px;">
      {$t('table.local_address')}
    </span>
  </div>

  <!-- Rows List -->
  <div class="flex-1 overflow-y-auto">
    {#if viewMode === 'flat'}
      <!-- FLAT TABLE VIEW -->
      {#if sortedConnections.length === 0}
        <div class="flex flex-col items-center justify-center h-64 gap-3 text-slate-500">
          <ShieldOff class="w-8 h-8 opacity-40" />
          <p class="text-[13px]">{$t('table.no_connections')}</p>
        </div>
      {:else}
        {#each sortedConnections as conn, i}
          {@const bg = i % 2 === 0 ? 'var(--bg-row-a)' : 'var(--bg-row-b)'}
          {@const color = avatarColor(conn.process_name)}
          <button
            type="button"
            onclick={() => (selectedConn = conn)}
            class="conn-row flex items-center px-4 py-2 border-b w-full text-left cursor-pointer"
            style="background:{bg}; border-color:var(--border);"
          >
            <!-- Application -->
            <div class="flex items-center gap-2.5 min-w-0" style="flex: 2.8;">
              <div class="flex items-center justify-center w-6 h-6 rounded text-[11px] font-bold shrink-0 uppercase shadow-sm"
                style="background:{color}20; color:{color}; border:1px solid {color}40;">
                {conn.process_name.charAt(0)}
              </div>
              <div class="min-w-0">
                <div class="text-[13px] font-medium text-slate-200 truncate">{conn.process_name}</div>
                <div class="text-[11px] font-mono text-slate-500 truncate">PID {conn.pid}</div>
              </div>
            </div>

            <!-- Remote Destination (Domain / IP) -->
            <div class="min-w-0" style="flex: 3.2;">
              <div class="text-[13px] font-medium truncate text-blue-400">
                {conn.domain !== '*' ? conn.domain : conn.remote}
              </div>
              {#if conn.domain !== conn.remote && conn.remote !== '*:*'}
                <div class="text-[11px] font-mono text-slate-500 truncate">{conn.remote}</div>
              {/if}
            </div>

            <!-- Protocol + Port -->
            <div class="flex items-center justify-center gap-1.5" style="flex: 1.2;">
              <span class="px-1.5 py-0.5 rounded text-[11px] font-semibold {protoPill(conn.protocol)}">{conn.protocol}</span>
              <span class="font-mono text-[12px] text-slate-400">{remotePort(conn.remote)}</span>
            </div>

            <!-- Status -->
            <div class="flex justify-center" style="flex: 1.2;">
              <span class="px-2 py-0.5 rounded text-[11px] font-medium {statusPill(conn.state)}">{conn.state}</span>
            </div>

            <!-- Local Address -->
            <div class="font-mono text-[11px] text-right text-slate-400 truncate pr-3" style="flex: 1.8;">
              {conn.local}
            </div>
          </button>
        {/each}
      {/if}

    {:else}
      <!-- GROUPED BY PROCESS VIEW -->
      {#if processGroups.length === 0}
        <div class="flex flex-col items-center justify-center h-64 gap-3 text-slate-500">
          <ShieldOff class="w-8 h-8 opacity-40" />
          <p class="text-[13px]">{$t('table.no_connections')}</p>
        </div>
      {:else}
        {#each processGroups as group}
          {@const isExpanded = expandedPids.has(group.pid)}
          {@const color = avatarColor(group.process_name)}
          <div class="border-b" style="border-color:var(--border);">
            <!-- Group Header Row -->
            <button
              type="button"
              onclick={() => toggleGroup(group.pid)}
              class="flex items-center px-4 py-2.5 w-full text-left transition-colors cursor-pointer"
              style="background:rgba(255,255,255,0.02);"
            >
              <div class="flex items-center gap-2" style="flex: 2.8;">
                {#if isExpanded}
                  <ChevronDown class="w-4 h-4 text-slate-400" />
                {:else}
                  <ChevronRight class="w-4 h-4 text-slate-400" />
                {/if}
                <div class="flex items-center justify-center w-6 h-6 rounded text-[11px] font-bold shrink-0 uppercase"
                  style="background:{color}20; color:{color}; border:1px solid {color}40;">
                  {group.process_name.charAt(0)}
                </div>
                <div class="min-w-0">
                  <span class="text-[13px] font-semibold text-slate-200">{group.process_name}</span>
                  <span class="text-[11px] font-mono text-slate-500 ml-2">PID {group.pid}</span>
                </div>
              </div>

              <!-- Domains list preview -->
              <div class="text-[12px] text-slate-400 truncate" style="flex: 3.2;">
                {#if group.total_remote_domains.length > 0}
                  {group.total_remote_domains.slice(0, 3).join(', ')}{group.total_remote_domains.length > 3 ? ` +${group.total_remote_domains.length - 3} more` : ''}
                {:else}
                  <span class="text-slate-600">—</span>
                {/if}
              </div>

              <!-- Sockets count badge -->
              <div class="flex items-center justify-center" style="flex: 1.2;">
                <span class="px-2 py-0.5 rounded-full text-[11px] font-mono font-medium bg-blue-500/10 text-blue-400 border border-blue-500/20">
                  {group.connections.length} sockets
                </span>
              </div>

              <!-- Active count -->
              <div class="flex justify-center" style="flex: 1.2;">
                <span class="px-2 py-0.5 rounded text-[11px] font-medium pill-established">
                  {group.active_count} active
                </span>
              </div>

              <div class="text-[11px] text-right text-slate-500 pr-3 truncate" style="flex: 1.8;">
                {group.process_path || 'System Process'}
              </div>
            </button>

            <!-- Expanded Sub-rows for this process -->
            {#if isExpanded}
              <div class="pl-8 bg-black/20 border-t" style="border-color:rgba(255,255,255,0.03);">
                {#each group.connections as conn}
                  <button
                    type="button"
                    onclick={() => (selectedConn = conn)}
                    class="conn-row flex items-center px-4 py-2 border-b w-full text-left cursor-pointer"
                    style="border-color:rgba(255,255,255,0.04);"
                  >
                    <div style="flex: 2.8;" class="pl-2">
                      <span class="text-[12px] text-slate-300 font-mono">{conn.local}</span>
                    </div>

                    <div style="flex: 3.2;" class="truncate text-blue-400 text-[12px] font-medium">
                      {conn.domain !== '*' ? conn.domain : conn.remote}
                    </div>

                    <div class="flex items-center justify-center gap-1" style="flex: 1.2;">
                      <span class="px-1.5 py-0.5 rounded text-[10px] font-semibold {protoPill(conn.protocol)}">{conn.protocol}</span>
                      <span class="font-mono text-[11px] text-slate-400">{remotePort(conn.remote)}</span>
                    </div>

                    <div class="flex justify-center" style="flex: 1.2;">
                      <span class="px-2 py-0.5 rounded text-[10px] font-medium {statusPill(conn.state)}">{conn.state}</span>
                    </div>

                    <div class="font-mono text-[11px] text-right text-slate-500 pr-3" style="flex: 1.8;">
                      {conn.remote}
                    </div>
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      {/if}
    {/if}
  </div>

  <!-- Detail Slide-Over / Drawer for clicked connection -->
  {#if selectedConn}
    <div class="absolute inset-y-0 right-0 w-96 shadow-2xl border-l flex flex-col z-20"
      style="background:var(--bg-panel); border-color:var(--border-light);">
      
      <!-- Drawer Header -->
      <div class="flex items-center justify-between px-5 py-4 border-b" style="border-color:var(--border);">
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="p-1.5 rounded-md bg-blue-500/10 text-blue-400 border border-blue-500/20">
            <Network class="w-4 h-4" />
          </div>
          <span class="font-bold text-[14px] text-slate-200 truncate">Connection Details</span>
        </div>
        <button
          onclick={() => (selectedConn = null)}
          class="p-1.5 rounded-md text-slate-400 hover:text-white hover:bg-white/10 transition-colors"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Drawer Body -->
      <div class="flex-1 overflow-y-auto p-5 space-y-4">
        <!-- Application card -->
        <div class="p-3.5 rounded-lg border bg-black/20" style="border-color:var(--border);">
          <div class="flex items-center gap-2 text-slate-400 text-[11px] uppercase font-semibold mb-2">
            <Cpu class="w-3.5 h-3.5" />
            <span>Process Info</span>
          </div>
          <div class="text-[14px] font-semibold text-slate-100">{selectedConn.process_name}</div>
          <div class="text-[12px] font-mono text-slate-400 mt-1">PID: {selectedConn.pid}</div>
          <div class="text-[11px] font-mono text-slate-500 mt-1 break-all bg-black/40 p-2 rounded border border-white/5">
            {selectedConn.process_path || 'N/A'}
          </div>
        </div>

        <!-- Remote Destination Card -->
        <div class="p-3.5 rounded-lg border bg-black/20" style="border-color:var(--border);">
          <div class="flex items-center gap-2 text-slate-400 text-[11px] uppercase font-semibold mb-2">
            <Globe class="w-3.5 h-3.5" />
            <span>Remote Destination</span>
          </div>
          <div class="text-[13px] font-medium text-blue-400 break-all">
            {selectedConn.domain !== '*' ? selectedConn.domain : 'Unresolved / Raw IP'}
          </div>
          
          <div class="flex items-center justify-between mt-3 pt-2 border-t border-white/5">
            <span class="text-[11px] font-mono text-slate-400">{selectedConn.remote}</span>
            <button
              onclick={() => copyText(selectedConn!.remote, 'remote')}
              class="flex items-center gap-1 px-2 py-1 rounded text-[11px] bg-white/5 hover:bg-white/10 text-slate-300 transition-colors border border-white/10"
            >
              {#if copiedField === 'remote'}
                <Check class="w-3 h-3 text-emerald-400" />
                <span class="text-emerald-400">Copied</span>
              {:else}
                <Copy class="w-3 h-3" />
                <span>Copy IP</span>
              {/if}
            </button>
          </div>
        </div>

        <!-- Socket Specs -->
        <div class="grid grid-cols-2 gap-2.5">
          <div class="p-3 rounded-lg border bg-black/20" style="border-color:var(--border);">
            <div class="text-[10px] uppercase font-semibold text-slate-400 mb-1">Protocol</div>
            <span class="px-2 py-0.5 rounded text-[11px] font-bold {protoPill(selectedConn.protocol)}">
              {selectedConn.protocol}
            </span>
          </div>
          <div class="p-3 rounded-lg border bg-black/20" style="border-color:var(--border);">
            <div class="text-[10px] uppercase font-semibold text-slate-400 mb-1">State</div>
            <span class="px-2 py-0.5 rounded text-[11px] font-bold {statusPill(selectedConn.state)}">
              {selectedConn.state}
            </span>
          </div>
        </div>

        <!-- Local Endpoint -->
        <div class="p-3.5 rounded-lg border bg-black/20" style="border-color:var(--border);">
          <div class="text-[10px] uppercase font-semibold text-slate-400 mb-1">Local Socket Endpoint</div>
          <div class="text-[12px] font-mono text-slate-300">{selectedConn.local}</div>
        </div>
      </div>

      <!-- Drawer Footer -->
      <div class="p-4 border-t flex justify-end" style="border-color:var(--border);">
        <button
          onclick={() => (selectedConn = null)}
          class="px-4 py-1.5 rounded-md text-[12px] font-medium bg-white/5 hover:bg-white/10 text-slate-300 border border-white/10 transition-colors"
        >
          Close
        </button>
      </div>
    </div>
  {/if}
</div>
