<script lang="ts">
  import type { EnrichedConnection, ProcessGroup } from '$lib/types/network';
  import { t } from 'svelte-i18n';
  import { BarChart3, PieChart, Network, Server } from 'lucide-svelte';

  let {
    connections = [],
    processGroups = []
  }: {
    connections?: EnrichedConnection[];
    processGroups?: ProcessGroup[];
  } = $props();

  // Top apps
  let topApps = $derived.by(() => {
    const max = Math.max(...processGroups.map(g => g.connections.length), 1);
    return processGroups.slice(0, 8).map(g => ({
      name: g.process_name,
      pid: g.pid,
      count: g.connections.length,
      pct: Math.round((g.connections.length / max) * 100)
    }));
  });

  // Protocol count
  let protocolCounts = $derived.by(() => {
    let tcp = 0;
    let udp = 0;
    for (const c of connections) {
      if (c.protocol === 'TCP') tcp++;
      else if (c.protocol === 'UDP') udp++;
    }
    const total = Math.max(tcp + udp, 1);
    return {
      tcp,
      udp,
      tcpPct: Math.round((tcp / total) * 100),
      udpPct: Math.round((udp / total) * 100)
    };
  });

  // Top ports
  let topPorts = $derived.by(() => {
    const map = new Map<string, number>();
    for (const c of connections) {
      const parts = c.remote.split(':');
      const port = parts.length > 1 ? parts[parts.length - 1] : '';
      if (port && port !== '*') {
        map.set(port, (map.get(port) || 0) + 1);
      }
    }
    return Array.from(map.entries())
      .map(([port, count]) => {
        let label = port;
        if (port === '443') label = '443 (HTTPS)';
        else if (port === '80') label = '80 (HTTP)';
        else if (port === '53') label = '53 (DNS)';
        else if (port === '22') label = '22 (SSH)';
        return { port, label, count };
      })
      .sort((a, b) => b.count - a.count)
      .slice(0, 6);
  });
</script>

<div class="flex-1 overflow-y-auto p-6 space-y-6 select-none">
  <div>
    <h2 class="text-[18px] font-bold text-slate-100 flex items-center gap-2">
      <BarChart3 class="w-5 h-5 text-blue-400" />
      {$t('stats.title')}
    </h2>
    <p class="text-[12px] text-slate-400 mt-1">Real-time socket distribution and endpoint analysis</p>
  </div>

  <!-- Top row: Protocol + Port Breakdown -->
  <div class="grid grid-cols-2 gap-5">
    <!-- Protocol distribution -->
    <div class="p-5 rounded-xl border bg-black/20" style="border-color:var(--border);">
      <div class="flex items-center justify-between mb-4">
        <span class="text-[13px] font-semibold text-slate-200 flex items-center gap-2">
          <PieChart class="w-4 h-4 text-indigo-400" />
          {$t('stats.protocol_dist')}
        </span>
        <span class="text-[11px] font-mono text-slate-400">{connections.length} total</span>
      </div>

      <div class="space-y-3">
        <!-- TCP -->
        <div>
          <div class="flex justify-between text-[12px] mb-1">
            <span class="text-blue-400 font-medium">TCP</span>
            <span class="font-mono text-slate-300">{protocolCounts.tcp} ({protocolCounts.tcpPct}%)</span>
          </div>
          <div class="h-2 rounded-full bg-slate-800 overflow-hidden">
            <div class="h-full bg-blue-500 rounded-full transition-all duration-300" style="width: {protocolCounts.tcpPct}%"></div>
          </div>
        </div>

        <!-- UDP -->
        <div>
          <div class="flex justify-between text-[12px] mb-1">
            <span class="text-purple-400 font-medium">UDP</span>
            <span class="font-mono text-slate-300">{protocolCounts.udp} ({protocolCounts.udpPct}%)</span>
          </div>
          <div class="h-2 rounded-full bg-slate-800 overflow-hidden">
            <div class="h-full bg-purple-500 rounded-full transition-all duration-300" style="width: {protocolCounts.udpPct}%"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Common Ports -->
    <div class="p-5 rounded-xl border bg-black/20" style="border-color:var(--border);">
      <div class="flex items-center justify-between mb-4">
        <span class="text-[13px] font-semibold text-slate-200 flex items-center gap-2">
          <Server class="w-4 h-4 text-emerald-400" />
          {$t('stats.popular_ports')}
        </span>
      </div>

      <div class="grid grid-cols-2 gap-2.5">
        {#each topPorts as portItem}
          <div class="p-2.5 rounded-lg border bg-black/30 flex items-center justify-between" style="border-color:var(--border);">
            <span class="text-[12px] font-medium text-slate-300">{portItem.label}</span>
            <span class="text-[12px] font-mono font-bold text-emerald-400">{portItem.count}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Bottom: Top Applications by Sockets -->
  <div class="p-5 rounded-xl border bg-black/20" style="border-color:var(--border);">
    <div class="flex items-center justify-between mb-4">
      <span class="text-[13px] font-semibold text-slate-200 flex items-center gap-2">
        <Network class="w-4 h-4 text-blue-400" />
        {$t('stats.top_apps')}
      </span>
    </div>

    <div class="space-y-3">
      {#each topApps as app}
        <div>
          <div class="flex justify-between text-[12px] mb-1">
            <span class="font-medium text-slate-200">{app.name} <span class="text-slate-500 font-mono text-[11px]">(PID {app.pid})</span></span>
            <span class="font-mono text-slate-400 font-semibold">{app.count} sockets</span>
          </div>
          <div class="h-2 rounded-full bg-slate-800 overflow-hidden">
            <div class="h-full bg-gradient-to-r from-blue-500 to-indigo-500 rounded-full transition-all duration-300" style="width: {app.pct}%"></div>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
