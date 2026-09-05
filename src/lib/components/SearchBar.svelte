<script lang="ts">
  import { Search, X } from 'lucide-svelte';
  import { t } from 'svelte-i18n';

  let {
    query = '',
    onQueryChange = () => {}
  }: {
    query?: string;
    onQueryChange?: (q: string) => void;
  } = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onQueryChange('');
    }
  }
</script>

<div class="relative flex-1 max-w-md">
  <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-slate-500">
    <Search class="w-4 h-4 text-slate-400" />
  </div>
  <input
    type="text"
    value={query}
    oninput={(e) => onQueryChange((e.target as HTMLInputElement).value)}
    onkeydown={handleKeydown}
    placeholder={$t('filters.search_placeholder')}
    class="w-full pl-9 pr-8 py-1.5 bg-slate-900/90 border border-slate-800 focus:border-cyan-500/50 rounded-lg text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:ring-1 focus:ring-cyan-500/30 transition-all font-sans"
  />
  {#if query}
    <button
      onclick={() => onQueryChange('')}
      class="absolute inset-y-0 right-0 pr-2.5 flex items-center text-slate-500 hover:text-slate-300"
    >
      <X class="w-3.5 h-3.5" />
    </button>
  {/if}
</div>
