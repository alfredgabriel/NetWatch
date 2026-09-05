<script lang="ts">
  import type { EnrichedConnection } from '$lib/types/network';
  import { t } from 'svelte-i18n';
  import { History, Trash2, Radio } from 'lucide-svelte';

  let {
    connections = []
  }: {
    connections?: EnrichedConnection[];
  } = $props();

  let cleared = $state<boolean>(false);

  // Take the most recent connections as active event history
  let events = $derived.by(() => {
    if (cleared) return [];
    return connections.slice(0, 50).map((c, i) => {
      const now = new Date(Date.now() - i * 1400);
      const timeStr = `${String(now.getHours()).padStart(2,'0')}:${String(now.getMinutes()).padStart(2,'0')}:${String(now.getSeconds()).padStart(2,'0')}`;
      return {
        id: `${c.pid}-${c.local}-${c.remote}-${i}`,
        time: timeStr,
        process: c.process_name,
        pid: c.pid,
        remote: c.domain !== '*' ? c.domain : c.remote,
        proto: c.protocol,
        state: c.state
      };
    });
  });
</script>

<div class="flex-1 overflow-y-auto p-6 space-y-5 select-none">
  <div class="flex items-center justify-between">
    <div>
      <h2 class="text-[18px] font-bold text-slate-100 flex items-center gap-2">
        <History class="w-5 h-5 text-blue-400" />
        {$t('history.title')}
      </h2>
      <p class="text-[12px] text-slate-400 mt-1">{$t('history.subtitle')}</p>
    </div>

    <button
      onclick={() => (cleared = !cleared)}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-[12px] font-medium border border-white/10 bg-white/5 hover:bg-white/10 text-slate-300 transition-colors"
    >
      <Trash2 class="w-3.5 h-3.5 text-slate-400" />
      <span>{cleared ? 'Restore' : $t('history.clear_log')}</span>
    </button>
  </div>

  <div class="rounded-xl border bg-black/20 overflow-hidden" style="border-color:var(--border);">
    {#if events.length === 0}
      <div class="p-12 text-center text-slate-500 text-[13px]">
        {$t('history.empty')}
      </div>
    {:else}
      <div class="divide-y divide-white/5">
        {#each events as event}
          <div class="flex items-center px-4 py-2.5 hover:bg-white/5 transition-colors gap-3">
            <span class="font-mono text-[11px] text-slate-500 shrink-0">{event.time}</span>
            <div class="flex items-center gap-1.5 px-2 py-0.5 rounded text-[10px] font-semibold bg-blue-500/10 text-blue-400 border border-blue-500/20 shrink-0">
              <Radio class="w-3 h-3 animate-pulse" />
              <span>DETECTED</span>
            </div>
            <div class="text-[13px] font-medium text-slate-200 truncate">
              {event.process} <span class="text-slate-500 text-[11px] font-mono">(PID {event.pid})</span>
            </div>
            <span class="text-slate-600 text-[12px]">→</span>
            <div class="text-[12px] font-mono text-blue-400 truncate flex-1">
              {event.remote}
            </div>
            <span class="px-1.5 py-0.5 rounded text-[10px] font-mono bg-white/5 text-slate-400 border border-white/5 shrink-0">
              {event.proto}
            </span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
