<script lang="ts">
  import type { ProcessGroup } from '$lib/types/network';
  import ProcessRow from './ProcessRow.svelte';
  import { t } from 'svelte-i18n';
  import { ShieldOff } from 'lucide-svelte';

  let {
    groups = []
  }: {
    groups?: ProcessGroup[];
  } = $props();

  let expandedPids = $state<Set<number>>(new Set());

  function toggleExpand(pid: number) {
    const next = new Set(expandedPids);
    if (next.has(pid)) {
      next.delete(pid);
    } else {
      next.add(pid);
    }
    expandedPids = next;
  }
</script>

<div class="flex-1 overflow-y-auto bg-[#0a0d14]">
  {#if groups.length === 0}
    <div class="h-64 flex flex-col items-center justify-center text-slate-500 gap-3">
      <div class="p-4 rounded-full bg-slate-900 border border-slate-800 text-slate-600">
        <ShieldOff class="w-8 h-8" />
      </div>
      <p class="text-sm font-medium">{$t('table.no_connections')}</p>
    </div>
  {:else}
    <div class="divide-y divide-slate-800/60">
      {#each groups as group (group.pid)}
        <ProcessRow
          group={group}
          isExpanded={expandedPids.has(group.pid)}
          onToggleExpand={() => toggleExpand(group.pid)}
        />
      {/each}
    </div>
  {/if}
</div>
