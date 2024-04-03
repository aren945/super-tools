import { defineStore } from "pinia";
import { VNodeRef } from "vue";

// 显示视图的属性
export interface ViewInterface {
  ref: VNodeRef;
  key: string;
  rect: DOMRect;
}

// 显示的视图列表
export interface MainStateInterface {
  views: ViewInterface[];
}

export const useMainStore = defineStore("main-store", {
  state(): MainStateInterface {
    return {
      views: [],
    };
  },

  actions: {
    addViews(key: string, rect: DOMRect, ref: Element) {
      this.views.push({
        key,
        rect,
        ref,
      });
    },
    removeViews(key: string) {
      const index = this.views.findIndex((view) => view.key === key);
      if (index > -1) {
        this.views.splice(index, 1);
      }
    },
  },
});
