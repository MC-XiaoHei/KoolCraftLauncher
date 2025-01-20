import "vuetify/styles";
import "virtual:uno.css";
import "./app.scss";
import 'overlayscrollbars/overlayscrollbars.css';
import App from "./App.vue";
import { i18n } from "./plugins/i18n";
import { pinia } from "./plugins/pinia";
import { router } from "./plugins/router";
import { vuetify } from "./plugins/vuetify";

import { OverlayScrollbars, ClickScrollPlugin } from 'overlayscrollbars';
OverlayScrollbars.plugin(ClickScrollPlugin);

createApp(App)
    .use(router)
    .use(i18n)
    .use(vuetify)
    .use(pinia)
    .mount("#app");
