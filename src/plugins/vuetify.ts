import { createVuetify, ThemeDefinition } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi-svg";

const defaultDarkTheme: ThemeDefinition = {
  dark: true,
  colors: {
    background: "#121212",
    surface: "#1E1E1E",
    primary: "#E0E0E0",
  },
};

export const vuetify = createVuetify({
  theme: {
    defaultTheme: "defaultDarkTheme",
    themes: {
      defaultDarkTheme,
    },
    variations: {
      colors: ["surface", "primary"],
      lighten: 1,
      darken: 2,
    },
  },
  icons: {
    defaultSet: "mdi",
    aliases,
    sets: {
      mdi,
    },
  },
  defaults: {
    global: {
      ripple: true,
      elevation: 0,
    },
    VBtn: {
      color: "primary",
      size: "x-large",
      variant: "text",
    },
  },
});