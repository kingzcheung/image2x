import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from 'node:url'
import autoprefixer from 'autoprefixer'
import tailwind from 'tailwindcss'

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  css: {
    postcss: {
      plugins: [tailwind(), autoprefixer()],
    },
  },

  plugins: [vue()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  build:{
    target: process.env.TAURI_ENV_PLATFORM == 'windows'
        ? 'chrome105'
        : 'safari13',
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 5173,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

}));
