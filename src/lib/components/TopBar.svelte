<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { switchLanguage } from '$lib/i18n';

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

<div class="flex items-center justify-between gap-3 px-5 py-2.5 border-b shrink-0 select-none font-mono"
  style="background:#000000; border-color:#222222; min-height:52px;">
  
  <!-- Search input -->
  <div class="relative w-80 max-w-sm">
    <input
      type="text"
      value={searchQuery}
      oninput={(e) => onSearchChange((e.target as HTMLInputElement).value)}
      placeholder="[SEARCH SOCKET / IP / PROCESS...]"
      class="w-full px-3 py-1.5 text-[11px] border outline-none transition-all placeholder:text-[#555]"
      style="background:#050505; border-color:#333333; color:#ffffff;"
    />
  </div>

  <!-- Filter chips -->
  <div class="flex items-center gap-1 bg-[#050505] p-1 border border-[#222]">
    {#each filters as f}
      {@const isSelected = activeFilter === f.id}
      <button
        onclick={() => onFilterChange(f.id)}
        class="px-2.5 py-1 text-[10px] font-bold uppercase transition-all"
        style={isSelected
          ? 'background:#ffffff; color:#000000;'
          : 'color:#888888; background:transparent;'}
      >
        {$t(f.labelKey)}
      </button>
    {/each}
  </div>

  <!-- Right Actions: View Mode Switcher + Language + Pause -->
  <div class="flex items-center gap-2">
    <!-- View Switcher (Flat vs Grouped) -->
    <div class="flex items-center bg-[#050505] p-0.5 border border-[#222]">
      <button
        onclick={() => onViewModeChange('flat')}
        title={$t('table.view_flat')}
        class="px-2 py-1 text-[10px] font-bold uppercase transition-all"
        style={viewMode === 'flat' ? 'background:#ffffff; color:#000000;' : 'color:#888888;'}
      >
        FLAT
      </button>
      <button
        onclick={() => onViewModeChange('grouped')}
        title={$t('table.view_grouped')}
        class="px-2 py-1 text-[10px] font-bold uppercase transition-all"
        style={viewMode === 'grouped' ? 'background:#ffffff; color:#000000;' : 'color:#888888;'}
      >
        GROUPED
      </button>
    </div>

    <!-- Language toggle -->
    <button
      onclick={toggleLang}
      class="px-2.5 py-1 text-[10px] font-bold border transition-colors"
      style="color:#ffffff; border-color:#333; background:#050505;"
    >
      [{$locale === 'es' ? 'ES' : 'EN'}]
    </button>

    <!-- Pause/Resume button -->
    <button
      onclick={onTogglePause}
      class="px-3 py-1 text-[10px] font-bold uppercase border transition-all"
      style={isPaused
        ? 'color:#000000; border-color:#ffffff; background:#ffffff;'
        : 'color:#ffffff; border-color:#ffffff; background:#000000;'}
    >
      {#if isPaused}
        [► RESUME]
      {:else}
        [❚❚ PAUSE]
      {/if}
    </button>
  </div>
</div>
