import "vuetify/styles";
import "virtual:uno.css";
import App from "./App.vue";
import { i18n } from "./plugins/i18n.ts";
import { pinia } from "./plugins/pinia.ts";
import { vuetify } from "./plugins/vuetify.ts";
import { router } from "./router";
import "./store/tauri-store.ts";

createApp(App)
    .use(router)
    .use(i18n)
    .use(vuetify)
    .use(pinia)
    .mount("#app");
