<script lang="ts">
  import { locale } from 'svelte-i18n';
  import { switchLanguage } from '$lib/i18n';
  import { Globe } from 'lucide-svelte';

  function toggleLanguage() {
    const current = $locale;
    const next = current === 'es' ? 'en' : 'es';
    switchLanguage(next);
  }
</script>

<button
  onclick={toggleLanguage}
  class="flex items-center gap-1.5 px-2.5 py-1 text-xs font-medium text-slate-300 bg-slate-800/80 hover:bg-slate-700/80 border border-slate-700/50 rounded-md transition-colors shadow-sm"
  title="Switch Language / Cambiar Idioma"
>
  <Globe class="w-3.5 h-3.5 text-cyan-400" />
  <span class="uppercase tracking-wider">{$locale ?? 'en'}</span>
</button>
