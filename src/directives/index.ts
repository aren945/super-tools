import { LogicalSize, appWindow } from "@tauri-apps/api/window";
import { App, nextTick } from "vue";

export const registerDirectives = (app: App) => {
  app.directive("auto-window-size", {
    updated(el) {
      const width = (el as HTMLDivElement).clientWidth;
      const height = (el as HTMLDivElement).clientHeight;
      nextTick(async () => {
        await appWindow.setSize(new LogicalSize(width, height));
      });
    },
  });
};
