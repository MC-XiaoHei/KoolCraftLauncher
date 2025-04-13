import VueI18nPlugin from "@intlify/unplugin-vue-i18n/vite";
import TailwindCSS from "@tailwindcss/vite";
import Vue from "@vitejs/plugin-vue";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import VueRouter from "unplugin-vue-router/vite";
import PackageVersion from 'vite-plugin-package-version';
import { defineConfig } from "vite";
import VueDevTools from "vite-plugin-vue-devtools";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    VueRouter(),
    Vue(),
    VueI18nPlugin({
      include: resolve(dirname(fileURLToPath(import.meta.url)), "./src/locales/**"),
    }),
    TailwindCSS(),
    AutoImport({
      imports: [
        "vue",
        "vue-router",
      ],
      dts: true,
    }),
    Components({
      dts: true,
    }),
    PackageVersion(),
    VueDevTools(),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
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
  resolve: {
    alias: [
      {find: "@", replacement: resolve(__dirname, "src")},
      {find: "@libs", replacement: resolve(__dirname, "src/lib")},
    ],
  },
}));
