import { UnwrapRef } from "vue";
import { RouteLocationRaw, Router } from "vue-router";

enum RoutingDirection {
  Go,
  Back,
}

const routingDirection = ref(RoutingDirection.Go);
export const routerRef = ref<Router | undefined>(undefined);

function getRouter(): UnwrapRef<Router> {
  return routerRef.value! as UnwrapRef<Router>;
}

export async function routeTo(to: RouteLocationRaw) {
  routingDirection.value = RoutingDirection.Go;
  await getRouter().push(to);
}

export async function routeBackTo(to: RouteLocationRaw) {
  routingDirection.value = RoutingDirection.Back;
  await getRouter().push(to);
}

export function routeBack() {
  routingDirection.value = RoutingDirection.Back;
  getRouter().back();
}

export function isRoutingBack(): boolean {
  return routingDirection.value === RoutingDirection.Back;
}