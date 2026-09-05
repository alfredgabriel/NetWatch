<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    text = '',
    children
  }: {
    text?: string;
    children?: Snippet;
  } = $props();
</script>

<div class="group relative inline-block">
  {@render children?.()}
  <div class="pointer-events-none absolute bottom-full left-1/2 -translate-x-1/2 mb-2 hidden group-hover:block z-50">
    <div class="bg-slate-900 text-slate-200 text-[11px] font-mono px-2 py-1 rounded shadow-lg border border-slate-700 whitespace-nowrap">
      {text}
    </div>
  </div>
</div>
