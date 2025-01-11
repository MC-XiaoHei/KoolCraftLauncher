import { createPersistedState } from "pinia-plugin-persistedstate";

function createPiniaInstance() {
  const instance = createPinia();

  instance.use(createPersistedState({}));

  return instance;
}

export const pinia = createPiniaInstance();