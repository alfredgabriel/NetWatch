<script lang="ts">
  import { t } from 'svelte-i18n';
  import { ArrowUp, ArrowDown, Activity, Layers, CheckCircle2 } from 'lucide-svelte';
  import { bandwidthStore } from '$lib/stores/bandwidthStore.svelte';

  let {
    activeSocketsCount = 0,
    activeProcessesCount = 0,
    establishedCount = 0,
  }: {
    activeSocketsCount?: number;
    activeProcessesCount?: number;
    establishedCount?: number;
  } = $props();

  let downSpeed = $derived(bandwidthStore.formattedSpeed);
  let upSpeed = $derived(
    (bandwidthStore.currentSpeedKb * 0.35 >= 1024)
      ? `${((bandwidthStore.currentSpeedKb * 0.35) / 1024).toFixed(2)} MB/s`
      : `${(bandwidthStore.currentSpeedKb * 0.35).toFixed(1)} KB/s`
  );
</script>

<div class="grid grid-cols-5 shrink-0 border-b select-none" style="background:var(--bg-topbar); border-color:var(--border);">
  <!-- Upload -->
  <div class="flex items-center gap-3 px-5 py-2 border-r" style="border-color:var(--border);">
    <div class="p-1.5 rounded bg-blue-500/10 border border-blue-500/20 text-blue-400">
      <ArrowUp class="w-3.5 h-3.5" />
    </div>
    <div class="min-w-0">
      <div class="text-[10px] uppercase font-semibold tracking-wider text-slate-400">{$t('metrics.upload')}</div>
      <div class="text-[14px] font-bold font-mono text-blue-400 truncate">{upSpeed}</div>
    </div>
  </div>

  <!-- Download -->
  <div class="flex items-center gap-3 px-5 py-2 border-r" style="border-color:var(--border);">
    <div class="p-1.5 rounded bg-emerald-500/10 border border-emerald-500/20 text-emerald-400">
      <ArrowDown class="w-3.5 h-3.5" />
    </div>
    <div class="min-w-0">
      <div class="text-[10px] uppercase font-semibold tracking-wider text-slate-400">{$t('metrics.download')}</div>
      <div class="text-[14px] font-bold font-mono text-emerald-400 truncate">{downSpeed}</div>
    </div>
  </div>

  <!-- Active Sockets -->
  <div class="flex items-center gap-3 px-5 py-2 border-r" style="border-color:var(--border);">
    <div class="p-1.5 rounded bg-indigo-500/10 border border-indigo-500/20 text-indigo-400">
      <Activity class="w-3.5 h-3.5" />
    </div>
    <div class="min-w-0">
      <div class="text-[10px] uppercase font-semibold tracking-wider text-slate-400">{$t('metrics.active_connections')}</div>
      <div class="text-[14px] font-bold font-mono text-slate-200">{activeSocketsCount}</div>
    </div>
  </div>

  <!-- Active Apps -->
  <div class="flex items-center gap-3 px-5 py-2 border-r" style="border-color:var(--border);">
    <div class="p-1.5 rounded bg-purple-500/10 border border-purple-500/20 text-purple-400">
      <Layers class="w-3.5 h-3.5" />
    </div>
    <div class="min-w-0">
      <div class="text-[10px] uppercase font-semibold tracking-wider text-slate-400">{$t('metrics.active_processes')}</div>
      <div class="text-[14px] font-bold font-mono text-slate-200">{activeProcessesCount}</div>
    </div>
  </div>

  <!-- Established Sockets -->
  <div class="flex items-center gap-3 px-5 py-2">
    <div class="p-1.5 rounded bg-teal-500/10 border border-teal-500/20 text-teal-400">
      <CheckCircle2 class="w-3.5 h-3.5" />
    </div>
    <div class="min-w-0">
      <div class="text-[10px] uppercase font-semibold tracking-wider text-slate-400">{$t('status.established')}</div>
      <div class="text-[14px] font-bold font-mono text-teal-400">{establishedCount}</div>
    </div>
  </div>
</div>
