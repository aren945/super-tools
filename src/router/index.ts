import { RouteRecordRaw, createRouter, createWebHistory } from "vue-router";

const routerModules = import.meta.glob<{
  default: RouteRecordRaw;
}>("@/pages/**/router.ts", {
  eager: true,
});

const routeModuleList: RouteRecordRaw[] = [];
Object.keys(routerModules).forEach((key) => {
  const mod = routerModules[key].default || {};
  const modList = Array.isArray(mod) ? [...mod] : [mod];
  routeModuleList.unshift(...modList);
});

const router = createRouter({
  history: createWebHistory(),
  routes: routeModuleList,
});

export default router;