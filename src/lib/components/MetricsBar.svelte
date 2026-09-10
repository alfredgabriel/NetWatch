<script lang="ts">
  import { t } from 'svelte-i18n';
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

<div class="grid grid-cols-5 shrink-0 border-b select-none font-mono" style="background:#000000; border-color:#222222;">
  <!-- Upload -->
  <div class="flex items-center gap-3 px-5 py-2.5 border-r border-[#222]">
    <div class="min-w-0">
      <div class="text-[9px] uppercase font-bold tracking-wider text-[#666]">// {$t('metrics.upload')}</div>
      <div class="text-[13px] font-bold text-white truncate">{upSpeed}</div>
    </div>
  </div>

  <!-- Download -->
  <div class="flex items-center gap-3 px-5 py-2.5 border-r border-[#222]">
    <div class="min-w-0">
      <div class="text-[9px] uppercase font-bold tracking-wider text-[#666]">// {$t('metrics.download')}</div>
      <div class="text-[13px] font-bold text-white truncate">{downSpeed}</div>
    </div>
  </div>

  <!-- Active Sockets -->
  <div class="flex items-center gap-3 px-5 py-2.5 border-r border-[#222]">
    <div class="min-w-0">
      <div class="text-[9px] uppercase font-bold tracking-wider text-[#666]">// {$t('metrics.active_connections')}</div>
      <div class="text-[13px] font-bold text-white">{activeSocketsCount}</div>
    </div>
  </div>

  <!-- Active Apps -->
  <div class="flex items-center gap-3 px-5 py-2.5 border-r border-[#222]">
    <div class="min-w-0">
      <div class="text-[9px] uppercase font-bold tracking-wider text-[#666]">// {$t('metrics.active_processes')}</div>
      <div class="text-[13px] font-bold text-white">{activeProcessesCount}</div>
    </div>
  </div>

  <!-- Established Sockets -->
  <div class="flex items-center gap-3 px-5 py-2.5">
    <div class="min-w-0">
      <div class="text-[9px] uppercase font-bold tracking-wider text-[#666]">// {$t('status.established')}</div>
      <div class="text-[13px] font-bold text-white">{establishedCount}</div>
    </div>
  </div>
</div>
