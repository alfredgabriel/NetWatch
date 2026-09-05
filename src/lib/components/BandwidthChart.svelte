<script lang="ts">
  import { bandwidthStore } from '$lib/stores/bandwidthStore.svelte';

  let points = $derived.by<string>(() => {
    const data = bandwidthStore.history;
    const max = Math.max(...data, 20);
    const width = 180;
    const height = 36;
    const step = width / (data.length - 1);

    return data
      .map((val, i) => {
        const x = i * step;
        const y = height - (val / max) * height;
        return `${x},${y}`;
      })
      .join(' ');
  });
</script>

<div class="flex items-center gap-3 bg-slate-900/60 border border-slate-800/80 px-3 py-1.5 rounded-lg">
  <div class="flex flex-col text-[11px] font-mono">
    <span class="text-slate-400 font-sans text-[10px] uppercase">Live Traffic</span>
    <span class="text-cyan-400 font-bold">{bandwidthStore.formattedSpeed}</span>
  </div>

  <svg class="w-24 h-8 overflow-visible" viewBox="0 0 180 36">
    <defs>
      <linearGradient id="chartGradient" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#00f2ff" stop-opacity="0.4" />
        <stop offset="100%" stop-color="#00f2ff" stop-opacity="0.0" />
      </linearGradient>
    </defs>
    <polyline
      fill="none"
      stroke="#00f2ff"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      points={points}
    />
  </svg>
</div>
