import "vuetify/styles";
import "virtual:uno.css";
import "./app.scss";
import "overlayscrollbars/overlayscrollbars.css";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { ClickScrollPlugin, OverlayScrollbars } from "overlayscrollbars";
import App from "./App.vue";
import { i18n } from "./plugins/i18n";
import { pinia } from "./plugins/pinia";
import { router } from "./plugins/router";
import { vuetify } from "./plugins/vuetify";

OverlayScrollbars.plugin(ClickScrollPlugin);

createApp(App)
    .use(router)
    .use(i18n)
    .use(vuetify)
    .use(pinia)
    .mount("#app");

getCurrentWebviewWindow().show().then();