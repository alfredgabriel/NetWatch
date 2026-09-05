<script lang="ts">
  import type { ProcessGroup } from '$lib/types/network';
  import ConnectionDetail from './ConnectionDetail.svelte';
  import { ChevronRight, ChevronDown, Cpu, Network } from 'lucide-svelte';

  let {
    group,
    isExpanded = false,
    onToggleExpand = () => {}
  }: {
    group: ProcessGroup;
    isExpanded?: boolean;
    onToggleExpand?: () => void;
  } = $props();
</script>

<div class="border-b border-slate-800/60 bg-slate-900/30 hover:bg-slate-800/40 transition-colors">
  <button
    onclick={onToggleExpand}
    class="w-full py-3 px-4 flex items-center justify-between text-left focus:outline-none"
  >
    <div class="flex items-center gap-3">
      <div class="text-slate-500">
        {#if isExpanded}
          <ChevronDown class="w-4 h-4 text-cyan-400" />
        {:else}
          <ChevronRight class="w-4 h-4" />
        {/if}
      </div>

      <div class="w-8 h-8 rounded bg-slate-800 border border-slate-700/60 flex items-center justify-center text-cyan-400 shadow-sm">
        <Cpu class="w-4 h-4" />
      </div>

      <div>
        <div class="flex items-center gap-2">
          <span class="font-semibold text-sm text-slate-100">{group.process_name}</span>
          <span class="text-[10px] font-mono px-1.5 py-0.2 bg-slate-800 text-slate-400 border border-slate-700 rounded">
            PID {group.pid}
          </span>
        </div>
        <p class="text-[11px] text-slate-400 truncate max-w-xs md:max-w-md font-mono mt-0.5" title={group.process_path}>
          {group.process_path || 'Executable Path N/A'}
        </p>
      </div>
    </div>

    <div class="flex items-center gap-4">
      <div class="flex items-center gap-1.5 text-xs text-slate-300 bg-slate-800/60 border border-slate-700/40 px-2.5 py-1 rounded-md">
        <Network class="w-3.5 h-3.5 text-indigo-400" />
        <span>{group.connections.length} socket{group.connections.length === 1 ? '' : 's'}</span>
      </div>

      {#if group.total_remote_domains.length > 0}
        <span class="text-xs font-mono text-cyan-400 hidden lg:inline bg-cyan-950/40 px-2 py-0.5 rounded border border-cyan-800/40">
          {group.total_remote_domains.length} domain{group.total_remote_domains.length === 1 ? '' : 's'}
        </span>
      {/if}
    </div>
  </button>

  {#if isExpanded}
    <div class="bg-slate-950/70 border-t border-slate-800/80 divide-y divide-slate-800/40 pl-6">
      {#each group.connections as conn}
        <ConnectionDetail connection={conn} />
      {/each}
    </div>
  {/if}
</div>
