// 全局事件监听
import { listen } from "@tauri-apps/api/event";
import { Plugin } from "vue";
import { ViewInterface, useMainStore } from "./store/main";
import { appWindow } from "@tauri-apps/api/window";

export interface TARUI_EVENT_MOUSE_POSITION {
  x: number;
  y: number;
}

// 判断鼠标是否在视图上面
const mouseIsInView = (
  mousePos: TARUI_EVENT_MOUSE_POSITION,
  view: ViewInterface,
) => {
  const boundaryX = view.rect.x + view.rect.width;
  const boundaryY = view.rect.y + view.rect.height;
  if (mousePos.x < view.rect.x || mousePos.x > boundaryX) return false;
  if (mousePos.y < view.rect.y || mousePos.y > boundaryY) return false;
  return true;
};

// 获取鼠标所在的第一个视图
const mouseIsInFirstView = (
  data: TARUI_EVENT_MOUSE_POSITION,
): ViewInterface | undefined => {
  const store = useMainStore();
  let view;
  for (let i = 0; i < store.views.length; i++) {
    const innerView = store.views[i];
    // 如果鼠标在某一个视图内，则跳出循环
    if (mouseIsInView(data, innerView)) {
      view = innerView;
      break;
    }
  }
  return view;
};

// 根据鼠标位置设置鼠标的穿透状态
const setTauriWindowIgnoreCursorEventsByPosition = async (
  data: TARUI_EVENT_MOUSE_POSITION,
) => {
  const view = !!mouseIsInFirstView(data);
  await appWindow.setIgnoreCursorEvents(!view);
};

export const init = () => {
  listen<TARUI_EVENT_MOUSE_POSITION>("mouse-position", async (data) => {
    const isFocused = await appWindow.isFocused();
    if (isFocused) {
      setTauriWindowIgnoreCursorEventsByPosition(data.payload);
    }
  });
  listen<TARUI_EVENT_MOUSE_POSITION>("mouse-click", async (data) => {
    const isFocused = await appWindow.isFocused();
    if (isFocused) {
      const hasMouseInView = !!mouseIsInFirstView(data.payload);
      if (!hasMouseInView) {
        appWindow.close();
      } else {
        appWindow.setFocus();
      }
    }
  });
};

export const globalEventPlugin = (): Plugin => {
  return {
    install() {
      init();
    },
  };
};
