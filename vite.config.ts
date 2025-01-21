import Vue from "@vitejs/plugin-vue";
import UnoCSS from "unocss/vite";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import VueRouter from "unplugin-vue-router/vite";
import { defineConfig } from "vite";
import ViteCompression from "vite-plugin-compression";
import VueDevTools from "vite-plugin-vue-devtools";
import Vuetify from "vite-plugin-vuetify";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    strictPort: true,
    host: host || false,
    port: 5173,
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    target:
        process.env.TAURI_ENV_PLATFORM == "windows"
            ? "chrome90"
            : "safari15",
    minify: !process.env.TAURI_ENV_DEBUG ? "terser" : false,
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
    },
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    chunkSizeWarningLimit: 1000,
  },
  plugins: [
    UnoCSS(),
    VueRouter({
      importMode: "sync",
    }),
    Vue(),
    Vuetify({ autoImport: true }),
    VueDevTools({
      launchEditor: "idea",
    }),
    AutoImport({
      imports: [
        "vue",
        "vue-router",
        "vue-i18n",
        "pinia",
      ],
    }),
    Components({
      dts: true,
    }),
    ViteCompression({}),
  ],
});