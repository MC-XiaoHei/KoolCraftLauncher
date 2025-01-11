import { createI18n } from "vue-i18n";

export const i18n = createI18n({
  legacy: false,
  locale: "zh-CN",
  messages: {
    "zh-CN": (await import(`../i18n/zh-CN.json`)).default,
  },
});