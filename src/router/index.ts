import { window } from "@tauri-apps/api";
import { register } from "@tauri-apps/api/globalShortcut";
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

// 注册快捷键
const registerHotKey = async () => {
  await register("CommandOrControl+Shift+O", (shortcut) => {
    console.log("shortcut is:", shortcut);
    window.appWindow.show();
    window.appWindow.setFocus();
  });
};

router.beforeEach((to, from, next) => {
  registerHotKey();
  next();
});

export default router;
