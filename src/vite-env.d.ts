/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  // noinspection JSUnusedGlobalSymbols
  export default component;
}
