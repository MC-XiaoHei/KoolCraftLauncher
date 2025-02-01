import { createRouter, createWebHistory, NavigationFailure, RouteLocationRaw } from "vue-router";
import { handleHotUpdate, routes } from "vue-router/auto-routes";

export const routingBack = ref(false);

export const router = createRouter({
  history: createWebHistory(),
  routes,
});

let legacyBack = router.back;
router.back = () => {
  routingBack.value = true;
  legacyBack();
};

let legacyPush: (to: RouteLocationRaw) => Promise<NavigationFailure | void | undefined> = router.push;
router.push = (to: RouteLocationRaw) => {
  routingBack.value = false;
  return legacyPush(to);
};

if (import.meta.hot) {
  handleHotUpdate(router);
}