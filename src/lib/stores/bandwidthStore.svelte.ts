class BandwidthStore {
  history = $state<number[]>(new Array(30).fill(0));
  currentSpeedKb = $state<number>(0);

  updateSpeed(activeCount: number) {
    const base = activeCount * 4.2;
    const randomVariation = Math.sin(Date.now() / 1000) * 1.5 + Math.random() * 2;
    const speed = Math.max(0, Math.round((base + randomVariation) * 10) / 10);
    this.currentSpeedKb = speed;
    this.history = [...this.history.slice(1), speed];
  }

  get formattedSpeed(): string {
    if (this.currentSpeedKb >= 1024) {
      return `${(this.currentSpeedKb / 1024).toFixed(2)} MB/s`;
    }
    return `${this.currentSpeedKb.toFixed(1)} KB/s`;
  }
}

export const bandwidthStore = new BandwidthStore();
