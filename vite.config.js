import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [tailwindcss(), sveltekit()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: "127.0.0.1",
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
