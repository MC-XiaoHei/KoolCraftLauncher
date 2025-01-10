import { createPersistedState } from "pinia-plugin-persistedstate";
import { createVuetify } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi-svg";
import "vuetify/styles";
import "virtual:uno.css";
import App from "./App.vue";
import { router } from "./router";
import { mixedStore } from "./store/mixed-store.ts";
import "./store/tauri-store.ts";

const vuetify = createVuetify({
  icons: {
    defaultSet: "mdi",
    aliases,
    sets: {
      mdi,
    },
  },
});

const pinia = createPinia();
pinia.use(createPersistedState({
  storage: mixedStore,
}));

createApp(App)
    .use(router)
    .use(vuetify)
    .use(pinia)
    .mount("#app");
