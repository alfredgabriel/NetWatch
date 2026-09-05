<script lang="ts">
  import { t } from 'svelte-i18n';
  import LanguageToggle from './LanguageToggle.svelte';
  import { ShieldCheck, Pause, Play } from 'lucide-svelte';

  let {
    isPaused = false,
    onTogglePause = () => {}
  }: {
    isPaused?: boolean;
    onTogglePause?: () => void;
  } = $props();
</script>

<header class="h-14 bg-[#121824]/90 backdrop-blur-md border-b border-slate-800/80 px-4 flex items-center justify-between sticky top-0 z-50">
  <div class="flex items-center gap-3">
    <div class="relative flex items-center justify-center w-8 h-8 rounded-lg bg-cyan-950/80 border border-cyan-500/30 text-cyan-400 shadow-[0_0_12px_rgba(0,242,255,0.2)]">
      <ShieldCheck class="w-5 h-5 text-cyan-400" />
      <span class="absolute -top-0.5 -right-0.5 w-2 h-2 bg-emerald-400 rounded-full animate-ping"></span>
    </div>
    
    <div>
      <div class="flex items-center gap-2">
        <h1 class="font-bold text-base tracking-tight text-white flex items-center gap-1.5">
          {$t('app.name')}
          <span class="text-[10px] uppercase font-mono px-1.5 py-0.5 rounded bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
            {$t('app.version')}
          </span>
        </h1>
      </div>
      <p class="text-[11px] text-slate-400 -mt-0.5 font-medium">{$t('app.tagline')}</p>
    </div>
  </div>

  <div class="flex items-center gap-3">
    <div class="hidden sm:flex items-center gap-2 px-3 py-1 rounded-full bg-slate-900/60 border border-slate-800 text-xs font-mono">
      <span class={`w-2 h-2 rounded-full ${isPaused ? 'bg-amber-400' : 'bg-emerald-400 animate-pulse'}`}></span>
      <span class="text-slate-300">
        {isPaused ? 'PAUSED' : 'MONITORING'}
      </span>
    </div>

    <button
      onclick={onTogglePause}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700/60 transition-all shadow-sm"
    >
      {#if isPaused}
        <Play class="w-3.5 h-3.5 text-emerald-400 fill-emerald-400/20" />
        <span>{$t('actions.resume')}</span>
      {:else}
        <Pause class="w-3.5 h-3.5 text-amber-400 fill-amber-400/20" />
        <span>{$t('actions.pause')}</span>
      {/if}
    </button>

    <LanguageToggle />
  </div>
</header>
