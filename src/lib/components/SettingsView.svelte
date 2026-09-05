<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { switchLanguage } from '$lib/i18n';
  import { Settings, Globe, Shield, Clock, Terminal } from 'lucide-svelte';

  function changeLocale(loc: 'en' | 'es') {
    switchLanguage(loc);
  }
</script>

<div class="flex-1 overflow-y-auto p-6 max-w-3xl space-y-6 select-none">
  <div>
    <h2 class="text-[18px] font-bold text-slate-100 flex items-center gap-2">
      <Settings class="w-5 h-5 text-blue-400" />
      {$t('settings.title')}
    </h2>
    <p class="text-[12px] text-slate-400 mt-1">Preferences and system configuration</p>
  </div>

  <!-- General Settings Card -->
  <div class="p-5 rounded-xl border bg-black/20 space-y-5" style="border-color:var(--border);">
    <div class="text-[14px] font-semibold text-slate-200 border-b pb-2 flex items-center gap-2" style="border-color:var(--border);">
      <Globe class="w-4 h-4 text-indigo-400" />
      {$t('settings.general')}
    </div>

    <!-- Language setting -->
    <div class="flex items-center justify-between">
      <div>
        <div class="text-[13px] font-medium text-slate-200">{$t('settings.language_label')}</div>
        <div class="text-[11px] text-slate-400">Choose between English and Spanish</div>
      </div>
      <div class="flex items-center gap-2">
        <button
          onclick={() => changeLocale('en')}
          class="px-3 py-1.5 rounded-md text-[12px] font-medium border transition-all"
          style={$locale === 'en'
            ? 'background:var(--accent); color:#fff; border-color:var(--accent);'
            : 'background:rgba(255,255,255,0.04); color:var(--text-muted); border-color:var(--border);'}
        >
          English
        </button>
        <button
          onclick={() => changeLocale('es')}
          class="px-3 py-1.5 rounded-md text-[12px] font-medium border transition-all"
          style={$locale === 'es'
            ? 'background:var(--accent); color:#fff; border-color:var(--accent);'
            : 'background:rgba(255,255,255,0.04); color:var(--text-muted); border-color:var(--border);'}
        >
          Español
        </button>
      </div>
    </div>

    <!-- Polling cycle -->
    <div class="flex items-center justify-between pt-3 border-t" style="border-color:rgba(255,255,255,0.05);">
      <div>
        <div class="text-[13px] font-medium text-slate-200">{$t('settings.polling_label')}</div>
        <div class="text-[11px] text-slate-400">{$t('settings.polling_desc')}</div>
      </div>
      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded bg-black/40 border border-white/10 font-mono text-[12px] text-emerald-400">
        <Clock class="w-3.5 h-3.5" />
        <span>300ms</span>
      </div>
    </div>
  </div>

  <!-- About Card -->
  <div class="p-5 rounded-xl border bg-black/20 space-y-4" style="border-color:var(--border);">
    <div class="text-[14px] font-semibold text-slate-200 border-b pb-2 flex items-center gap-2" style="border-color:var(--border);">
      <Shield class="w-4 h-4 text-emerald-400" />
      {$t('settings.about_title')}
    </div>

    <p class="text-[12px] text-slate-300 leading-relaxed">
      {$t('settings.about_desc')}
    </p>

    <div class="grid grid-cols-2 gap-3 pt-2">
      <div class="p-3 rounded-lg border bg-black/30 text-[11px]" style="border-color:var(--border);">
        <span class="text-slate-400">Frontend Stack</span>
        <div class="font-mono text-slate-200 font-semibold mt-0.5">Svelte 5 + Tailwind CSS</div>
      </div>
      <div class="p-3 rounded-lg border bg-black/30 text-[11px]" style="border-color:var(--border);">
        <span class="text-slate-400">Core Engine</span>
        <div class="font-mono text-slate-200 font-semibold mt-0.5">Rust + Tokio + Windows NetAPI</div>
      </div>
    </div>
  </div>
</div>
