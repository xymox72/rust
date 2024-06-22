import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { viteStaticCopy } from 'vite-plugin-static-copy';
import path from 'path';
import { normalizePath } from 'vite';
// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(),
    viteStaticCopy({
      targets: [
        {
          src: normalizePath('./env'),
          dest: normalizePath('./dist/')
        }
      ]
    })
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 3000,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
