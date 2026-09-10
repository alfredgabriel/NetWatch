<script lang="ts">
  import { t } from 'svelte-i18n';
  import { Network, Activity, History, Settings, Zap } from 'lucide-svelte';
  import { bandwidthStore } from '$lib/stores/bandwidthStore.svelte';

  let {
    activeView = 'connections',
    onViewSelect = (_: string) => {}
  }: {
    activeView?: string;
    onViewSelect?: (view: string) => void;
  } = $props();

  const navItems = [
    { id: 'connections', num: '01', labelKey: 'nav.connections' },
    { id: 'stats',       num: '02', labelKey: 'nav.statistics' },
    { id: 'history',     num: '03', labelKey: 'nav.history' },
    { id: 'settings',    num: '04', labelKey: 'nav.settings' },
  ];

  let sparkPoints = $derived.by(() => {
    const d = bandwidthStore.history;
    const max = Math.max(...d, 1);
    const w = 172, h = 34, step = w / Math.max(d.length - 1, 1);
    return d.map((v, i) => `${(i * step).toFixed(1)},${(h - (v / max) * h).toFixed(1)}`).join(' ');
  });
</script>

<aside class="flex flex-col shrink-0 border-r select-none h-full font-mono" style="width:230px; background:#000000; border-color:#222222;">
  <!-- App Logo / Title -->
  <div class="flex items-center gap-3 px-5 py-4 border-b" style="border-color:#222222;">
    <div class="flex items-center justify-center w-7 h-7 border border-[#444] bg-[#0a0a0a]">
      <span class="text-[11px] font-bold text-white">[NW]</span>
    </div>
    <div class="flex flex-col">
      <span class="font-bold text-[13px] tracking-wider text-white">NET<span class="line-through text-[#666]">WATCH</span></span>
      <span class="text-[10px] text-[#666]">// SYS MONITOR</span>
    </div>
  </div>

  <!-- Navigation items -->
  <nav class="flex flex-col gap-1 p-3 flex-1 overflow-y-auto">
    {#each navItems as item}
      {@const isActive = activeView === item.id}
      <button
        onclick={() => onViewSelect(item.id)}
        class="nav-item flex items-center justify-between px-3 py-2 text-left text-[11px] font-bold tracking-wider uppercase transition-all"
        style={isActive
          ? 'background:#ffffff; color:#000000; border:1px solid #ffffff;'
          : 'color:#888888; background:transparent; border:1px solid transparent;'}
      >
        <span>{$t(item.labelKey)}</span>
        <span class="text-[10px] opacity-70">[{item.num}]</span>
      </button>
    {/each}
  </nav>

  <!-- Bottom Telemetry Sparkline -->
  <div class="p-4 border-t" style="border-color:#222222; background:#050505;">
    <div class="flex items-center justify-between mb-2">
      <span class="text-[10px] font-bold text-[#666] tracking-wider">// BANDWIDTH</span>
      <span class="text-[11px] font-bold text-white">
        {bandwidthStore.formattedSpeed}
      </span>
    </div>

    <!-- Live SVG Wave -->
    <div class="p-1 mb-2.5 overflow-hidden" style="background:#000000; border:1px solid #222222;">
      <svg width="100%" height="34" viewBox="0 0 172 34" preserveAspectRatio="none">
        <polyline
          fill="none"
          stroke="#ffffff"
          stroke-width="1.5"
          stroke-linecap="square"
          stroke-linejoin="miter"
          points={sparkPoints}
        />
      </svg>
    </div>

    <!-- LIVE badge -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-1.5 px-2 py-0.5 text-[9px] font-bold tracking-widest border border-white text-white bg-black">
        [MONITORING]
      </div>
      <span class="text-[10px] text-[#555]">300ms poll</span>
    </div>
  </div>
</aside>
