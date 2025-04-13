import { i18n } from "@/plugins/i18n.ts";
import { router } from "@/plugins/router.ts";
import { createApp } from "vue";
import App from "./App.vue";

import "./index.css";

createApp(App)
    .use(router)
    .use(i18n)
    .mount("#app");
