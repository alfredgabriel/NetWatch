<script lang="ts">
  import { t } from 'svelte-i18n';
  import SearchBar from './SearchBar.svelte';
  import { Layers, Zap, AlertTriangle, ShieldCheck } from 'lucide-svelte';

  let {
    activeFilter = 'all',
    searchQuery = '',
    onFilterChange = () => {},
    onSearchChange = () => {}
  }: {
    activeFilter?: 'all' | 'active' | 'new' | 'unresolved';
    searchQuery?: string;
    onFilterChange?: (f: 'all' | 'active' | 'new' | 'unresolved') => void;
    onSearchChange?: (q: string) => void;
  } = $props();

  const filters = [
    { id: 'all', key: 'filters.all', icon: Layers },
    { id: 'active', key: 'filters.active_only', icon: Zap },
    { id: 'new', key: 'filters.new_only', icon: AlertTriangle },
    { id: 'unresolved', key: 'filters.unresolved', icon: ShieldCheck }
  ] as const;
</script>

<div class="px-4 py-2.5 bg-[#0d121d] border-b border-slate-800/80 flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3">
  <!-- Filter Pills -->
  <div class="flex items-center gap-1 bg-slate-950/80 p-1 rounded-lg border border-slate-800/80 overflow-x-auto">
    {#each filters as f}
      {@const Icon = f.icon}
      <button
        onclick={() => onFilterChange(f.id)}
        class={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all whitespace-nowrap ${
          activeFilter === f.id
            ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/40 shadow-sm'
            : 'text-slate-400 hover:text-slate-200 hover:bg-slate-900/60 border border-transparent'
        }`}
      >
        <Icon class="w-3.5 h-3.5" />
        <span>{$t(f.key)}</span>
      </button>
    {/each}
  </div>

  <!-- Search Input -->
  <SearchBar query={searchQuery} onQueryChange={onSearchChange} />
</div>
