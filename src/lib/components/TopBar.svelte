<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { switchLanguage } from '$lib/i18n';
  import { Search, Pause, Play, Globe2, LayoutList, Layers } from 'lucide-svelte';

  let {
    searchQuery = '',
    isPaused = false,
    activeFilter = 'all',
    viewMode = 'flat',
    onSearchChange = (_: string) => {},
    onTogglePause = () => {},
    onFilterChange = (_: string) => {},
    onViewModeChange = (_: 'flat' | 'grouped') => {}
  }: {
    searchQuery?: string;
    isPaused?: boolean;
    activeFilter?: string;
    viewMode?: 'flat' | 'grouped';
    onSearchChange?: (q: string) => void;
    onTogglePause?: () => void;
    onFilterChange?: (f: string) => void;
    onViewModeChange?: (m: 'flat' | 'grouped') => void;
  } = $props();

  function toggleLang() {
    switchLanguage($locale === 'es' ? 'en' : 'es');
  }

  const filters = [
    { id: 'all', labelKey: 'filters.all' },
    { id: 'tcp', labelKey: 'filters.tcp_only' },
    { id: 'udp', labelKey: 'filters.udp_only' },
    { id: 'established', labelKey: 'filters.established' },
    { id: 'listen', labelKey: 'filters.listening' },
  ];
</script>

<div class="flex items-center justify-between gap-3 px-5 py-2.5 border-b shrink-0 select-none"
  style="background:var(--bg-topbar); border-color:var(--border); min-height:54px;">
  
  <!-- Search input -->
  <div class="relative w-80 max-w-sm">
    <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 pointer-events-none" style="color:var(--text-muted);" />
    <input
      type="text"
      value={searchQuery}
      oninput={(e) => onSearchChange((e.target as HTMLInputElement).value)}
      placeholder={$t('filters.search_placeholder')}
      class="w-full pl-8 pr-3 py-1.5 rounded-md text-[12px] border outline-none transition-all placeholder:text-slate-500"
      style="background:#090C15; border-color:var(--border); color:var(--text-main);"
    />
  </div>

  <!-- Filter chips -->
  <div class="flex items-center gap-1.5 bg-[#080B13] p-1 rounded-lg border" style="border-color:var(--border);">
    {#each filters as f}
      {@const isSelected = activeFilter === f.id}
      <button
        onclick={() => onFilterChange(f.id)}
        class="px-2.5 py-1 rounded-md text-[11px] font-medium transition-all"
        style={isSelected
          ? 'background:var(--accent); color:#ffffff; font-weight:600;'
          : 'color:var(--text-muted); background:transparent;'}
      >
        {$t(f.labelKey)}
      </button>
    {/each}
  </div>

  <!-- Right Actions: View Mode Switcher + Language + Pause -->
  <div class="flex items-center gap-2">
    <!-- View Switcher (Flat vs Grouped) -->
    <div class="flex items-center bg-[#080B13] p-0.5 rounded-md border" style="border-color:var(--border);">
      <button
        onclick={() => onViewModeChange('flat')}
        title={$t('table.view_flat')}
        class="p-1.5 rounded text-[11px] transition-all flex items-center gap-1"
        style={viewMode === 'flat' ? 'background:rgba(255,255,255,0.1); color:var(--text-main);' : 'color:var(--text-muted);'}
      >
        <LayoutList class="w-3.5 h-3.5" />
      </button>
      <button
        onclick={() => onViewModeChange('grouped')}
        title={$t('table.view_grouped')}
        class="p-1.5 rounded text-[11px] transition-all flex items-center gap-1"
        style={viewMode === 'grouped' ? 'background:rgba(255,255,255,0.1); color:var(--text-main);' : 'color:var(--text-muted);'}
      >
        <Layers class="w-3.5 h-3.5" />
      </button>
    </div>

    <!-- Language toggle -->
    <button
      onclick={toggleLang}
      class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-[11px] font-medium border transition-colors hover:border-slate-500"
      style="color:var(--text-muted); border-color:var(--border); background:rgba(255,255,255,0.03);"
    >
      <Globe2 class="w-3.5 h-3.5" />
      <span>{$locale === 'es' ? 'ES' : 'EN'}</span>
    </button>

    <!-- Pause/Resume button -->
    <button
      onclick={onTogglePause}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-[12px] font-medium border transition-all"
      style={isPaused
        ? 'color:#4ADE80; border-color:rgba(74,222,128,0.4); background:rgba(74,222,128,0.1);'
        : 'color:var(--text-main); border-color:var(--border); background:rgba(255,255,255,0.04);'}
    >
      {#if isPaused}
        <Play class="w-3.5 h-3.5 text-emerald-400" />
        <span>{$t('actions.resume')}</span>
      {:else}
        <Pause class="w-3.5 h-3.5 text-amber-400" />
        <span>{$t('actions.pause')}</span>
      {/if}
    </button>
  </div>
</div>
