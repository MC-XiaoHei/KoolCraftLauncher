import "vuetify/styles";
import "virtual:uno.css";
import App from "./App.vue";
import { i18n } from "./plugins/i18n";
import { pinia } from "./plugins/pinia";
import { router } from "./plugins/router";
import { vuetify } from "./plugins/vuetify";

createApp(App)
    .use(router)
    .use(i18n)
    .use(vuetify)
    .use(pinia)
    .mount("#app");
