<script lang="ts">
  import { t } from 'svelte-i18n';
  import { ArrowUpRight, ArrowDownLeft, Network, Cpu } from 'lucide-svelte';
  import BandwidthChart from './BandwidthChart.svelte';

  let {
    activeSocketsCount = 0,
    activeProcessesCount = 0,
    uploadSpeedFormatted = '0 KB/s',
    downloadSpeedFormatted = '0 KB/s',
    newEventsCount = 0
  }: {
    activeSocketsCount?: number;
    activeProcessesCount?: number;
    uploadSpeedFormatted?: string;
    downloadSpeedFormatted?: string;
    newEventsCount?: number;
  } = $props();
</script>

<div class="grid grid-cols-2 md:grid-cols-5 gap-3 p-4 bg-[#0d121d]/80 border-b border-slate-800/60">
  <!-- Upload Card -->
  <div class="glass-card p-3 rounded-lg flex items-center justify-between border border-slate-800/80">
    <div>
      <p class="text-[11px] font-medium text-slate-400 uppercase tracking-wider">{$t('metrics.upload')}</p>
      <p class="text-lg font-bold font-mono text-cyan-400 mt-0.5">{uploadSpeedFormatted}</p>
    </div>
    <div class="p-2 rounded-lg bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
      <ArrowUpRight class="w-4 h-4" />
    </div>
  </div>

  <!-- Download Card -->
  <div class="glass-card p-3 rounded-lg flex items-center justify-between border border-slate-800/80">
    <div>
      <p class="text-[11px] font-medium text-slate-400 uppercase tracking-wider">{$t('metrics.download')}</p>
      <p class="text-lg font-bold font-mono text-emerald-400 mt-0.5">{downloadSpeedFormatted}</p>
    </div>
    <div class="p-2 rounded-lg bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
      <ArrowDownLeft class="w-4 h-4" />
    </div>
  </div>

  <!-- Active Sockets Card -->
  <div class="glass-card p-3 rounded-lg flex items-center justify-between border border-slate-800/80">
    <div>
      <p class="text-[11px] font-medium text-slate-400 uppercase tracking-wider">{$t('metrics.active_connections')}</p>
      <p class="text-lg font-bold font-mono text-white mt-0.5">{activeSocketsCount}</p>
    </div>
    <div class="p-2 rounded-lg bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
      <Network class="w-4 h-4" />
    </div>
  </div>

  <!-- Active Processes & New Events Card -->
  <div class="glass-card p-3 rounded-lg flex items-center justify-between border border-slate-800/80">
    <div>
      <p class="text-[11px] font-medium text-slate-400 uppercase tracking-wider">{$t('metrics.active_processes')}</p>
      <div class="flex items-center gap-2 mt-0.5">
        <span class="text-lg font-bold font-mono text-purple-300">{activeProcessesCount}</span>
        {#if newEventsCount > 0}
          <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/30">
            +{newEventsCount} new
          </span>
        {/if}
      </div>
    </div>
    <div class="p-2 rounded-lg bg-purple-500/10 text-purple-400 border border-purple-500/20">
      <Cpu class="w-4 h-4" />
    </div>
  </div>

  <!-- Realtime Graph Card -->
  <div class="col-span-2 md:col-span-1 glass-card p-2 rounded-lg flex items-center justify-center border border-slate-800/80">
    <BandwidthChart />
  </div>
</div>
