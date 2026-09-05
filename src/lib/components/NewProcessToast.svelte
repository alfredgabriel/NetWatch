<script lang="ts">
  import { ShieldAlert, X } from 'lucide-svelte';

  let {
    processName = '',
    pid = 0,
    onDismiss = () => {}
  }: {
    processName?: string;
    pid?: number;
    onDismiss?: () => void;
  } = $props();
</script>

{#if processName}
  <div class="fixed bottom-4 right-4 z-50 max-w-sm glass-panel p-4 rounded-xl border border-amber-500/40 shadow-2xl animate-bounce-short">
    <div class="flex items-start justify-between gap-3">
      <div class="p-2 rounded-lg bg-amber-500/20 text-amber-400 border border-amber-500/30">
        <ShieldAlert class="w-5 h-5" />
      </div>

      <div class="flex-1">
        <h4 class="text-xs font-bold text-slate-100 uppercase tracking-wider">New Connection Outbound</h4>
        <p class="text-xs font-mono text-cyan-300 font-semibold mt-0.5">{processName} <span class="text-slate-500 font-normal">(PID {pid})</span></p>
        <p class="text-[11px] text-slate-400 mt-1">First outbound socket connection recorded in this session.</p>
      </div>

      <button onclick={onDismiss} class="text-slate-500 hover:text-slate-300">
        <X class="w-4 h-4" />
      </button>
    </div>
  </div>
{/if}
