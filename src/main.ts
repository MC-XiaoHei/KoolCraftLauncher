import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import { createRouter, createWebHistory } from "vue-router";
import { routes } from "vue-router/auto-routes";
import messages from '@intlify/unplugin-vue-i18n/messages'
import App from "./App.vue";

import "./index.css";

const router = createRouter({
  history: createWebHistory(),
  routes,
});


const i18n = createI18n({
  locale: 'zh-cn',
  messages
})

createApp(App)
    .use(router)
    .use(i18n)
    .mount("#app");
