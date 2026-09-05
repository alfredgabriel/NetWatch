<script lang="ts">
  import type { EnrichedConnection } from '$lib/types/network';
  import { Globe, ArrowRight } from 'lucide-svelte';

  let { connection }: { connection: EnrichedConnection } = $props();

  function getStatusColor(state: string) {
    switch (state) {
      case 'ESTABLISHED':
        return 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';
      case 'LISTEN':
        return 'bg-indigo-500/10 text-indigo-400 border-indigo-500/20';
      case 'SYN_SENT':
        return 'bg-amber-500/10 text-amber-400 border-amber-500/20';
      case 'TIME_WAIT':
      case 'CLOSE_WAIT':
        return 'bg-slate-500/10 text-slate-400 border-slate-500/20';
      default:
        return 'bg-cyan-500/10 text-cyan-400 border-cyan-500/20';
    }
  }
</script>

<div class="flex items-center justify-between py-2 px-4 bg-slate-950/40 hover:bg-slate-900/60 border-b border-slate-800/40 text-xs font-mono transition-colors">
  <div class="flex items-center gap-3 min-w-[240px]">
    <span class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-slate-800 text-slate-300 border border-slate-700">
      {connection.protocol}
    </span>
    <span class="text-slate-400">{connection.local}</span>
    <ArrowRight class="w-3 h-3 text-slate-600" />
    <div class="flex items-center gap-1.5">
      <Globe class="w-3.5 h-3.5 text-cyan-400" />
      <span class="text-cyan-300 font-semibold">{connection.domain || connection.remote}</span>
    </div>
  </div>

  <div class="flex items-center gap-4">
    {#if connection.domain && connection.domain !== connection.remote && connection.remote !== '*:*'}
      <span class="text-[11px] text-slate-500 hidden md:inline font-sans">
        ({connection.remote})
      </span>
    {/if}

    <span class={`px-2 py-0.5 rounded-full text-[10px] font-medium border ${getStatusColor(connection.state)}`}>
      {connection.state}
    </span>
  </div>
</div>
