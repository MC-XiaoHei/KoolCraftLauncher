import Vue from "@vitejs/plugin-vue";
import UnoCSS from "unocss/vite";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import VueRouter from "unplugin-vue-router/vite";
import { defineConfig } from "vite";
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
    minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
  plugins: [
    UnoCSS(),
    VueRouter({}),
    Vue(),
    Vuetify({ autoImport: true }),
    VueDevTools({
      launchEditor: "idea",
    }),
    AutoImport({
      imports: [
        "vue",
        "vue-router",
        "pinia",
      ],
    }),
    Components({
      dts: true,
    }),
  ],
});