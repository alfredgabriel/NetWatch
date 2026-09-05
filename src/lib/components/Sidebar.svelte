<script lang="ts">
  import { t } from 'svelte-i18n';
  import { Activity, BarChart3, History, Settings, ShieldCheck, Zap } from 'lucide-svelte';
  import { bandwidthStore } from '$lib/stores/bandwidthStore.svelte';

  let {
    activeView = 'live',
    onViewSelect = (_: string) => {}
  }: {
    activeView?: string;
    onViewSelect?: (view: string) => void;
  } = $props();

  const navItems = [
    { id: 'live',       icon: Activity,  labelKey: 'nav.live' },
    { id: 'statistics', icon: BarChart3, labelKey: 'nav.statistics' },
    { id: 'history',    icon: History,   labelKey: 'nav.history' },
    { id: 'settings',   icon: Settings,  labelKey: 'nav.settings' },
  ];

  // Sparkline points calculation
  let sparkPoints = $derived.by(() => {
    const d = bandwidthStore.history;
    const max = Math.max(...d, 1);
    const w = 172, h = 34, step = w / Math.max(d.length - 1, 1);
    return d.map((v, i) => `${(i * step).toFixed(1)},${(h - (v / max) * h).toFixed(1)}`).join(' ');
  });
</script>

<aside class="flex flex-col shrink-0 border-r select-none h-full" style="width:230px; background:var(--bg-sidebar); border-color:var(--border);">
  <!-- App Logo / Title -->
  <div class="flex items-center gap-3 px-5 py-4 border-b" style="border-color:var(--border);">
    <div class="flex items-center justify-center w-8 h-8 rounded-lg shadow-sm" style="background:var(--accent-dim); border: 1px solid rgba(59,130,246,0.3);">
      <ShieldCheck class="w-4.5 h-4.5" style="color:var(--accent);" />
    </div>
    <div class="flex flex-col">
      <span class="font-bold text-[14px] tracking-tight" style="color:var(--text-main);">NetWatch</span>
      <span class="text-[10px] font-mono" style="color:var(--text-muted);">v0.1.0 • Windows</span>
    </div>
  </div>

  <!-- Navigation items -->
  <nav class="flex flex-col gap-1 p-3 flex-1 overflow-y-auto">
    {#each navItems as item}
      {@const Icon = item.icon}
      {@const isActive = activeView === item.id}
      <button
        onclick={() => onViewSelect(item.id)}
        class="nav-item flex items-center gap-3 px-3.5 py-2.5 rounded-lg text-left text-[13px] font-medium transition-all"
        style={isActive
          ? 'background:var(--accent-dim); color:#93C5FD; border:1px solid rgba(59,130,246,0.25);'
          : 'color:var(--text-muted); border:1px solid transparent;'}
      >
        <Icon class="w-4 h-4 shrink-0" style={isActive ? 'color:#60A5FA;' : ''} />
        <span>{$t(item.labelKey)}</span>
      </button>
    {/each}
  </nav>

  <!-- Bottom Telemetry Sparkline -->
  <div class="p-4 border-t" style="border-color:var(--border); background:rgba(0,0,0,0.15);">
    <div class="flex items-center justify-between mb-2">
      <div class="flex items-center gap-1.5">
        <Zap class="w-3.5 h-3.5 text-blue-400" />
        <span class="text-[11px] font-medium" style="color:var(--text-muted);">Bandwidth</span>
      </div>
      <span class="text-[11px] font-mono font-semibold" style="color:#60A5FA;">
        {bandwidthStore.formattedSpeed}
      </span>
    </div>

    <!-- Live SVG Wave -->
    <div class="rounded-md p-1.5 mb-2.5 overflow-hidden" style="background:rgba(0,0,0,0.3); border:1px solid var(--border);">
      <svg width="100%" height="34" viewBox="0 0 172 34" preserveAspectRatio="none">
        <polyline
          fill="none"
          stroke="#3B82F6"
          stroke-width="1.75"
          stroke-linecap="round"
          stroke-linejoin="round"
          points={sparkPoints}
        />
      </svg>
    </div>

    <!-- LIVE badge -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[10px] font-bold tracking-wider"
        style="background:rgba(34,197,94,0.12); color:#4ADE80; border:1px solid rgba(34,197,94,0.3);">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-ping inline-block"></span>
        MONITORING
      </div>
      <span class="text-[10px] font-mono" style="color:var(--text-faint);">300ms poll</span>
    </div>
  </div>
</aside>
