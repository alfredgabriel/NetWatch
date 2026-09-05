<script lang="ts">
  import '../app.css';
  import type { Snippet } from 'svelte';
  import { setupI18n } from '$lib/i18n';
  import { isLoading } from 'svelte-i18n';

  let { children }: { children: Snippet } = $props();

  setupI18n();
</script>

{#if $isLoading}
  <div class="h-screen w-screen flex items-center justify-center text-sm" style="background:var(--bg-main); color:var(--text-muted);">
    <div class="flex items-center gap-3">
      <div class="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
      <span style="color:var(--text-main);">NetWatch...</span>
    </div>
  </div>
{:else}
  <div class="h-screen w-screen flex flex-col overflow-hidden select-none" style="background:var(--bg-main); color:var(--text-main);">
    {@render children()}
  </div>
{/if}
