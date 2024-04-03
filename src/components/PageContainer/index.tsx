import { SlotsType, defineComponent, onMounted, onUnmounted, ref } from "vue";
import s from "./index.module.less";
import { JSX } from "vue/jsx-runtime";
import { useMainStore } from "@/store/main";
import { createUuid } from "@/utils/uuid";

export default defineComponent({
  name: "PageContainer",
  slots: {} as SlotsType<{
    default: () => JSX.Element;
  }>,
  setup(_, { slots }) {
    const store = useMainStore();
    const wrapper = ref<Element>();
    let uuid = "";

    onMounted(() => {
      console.log();
      uuid = createUuid();
      const firstChilde = wrapper.value?.firstElementChild;
      if (!firstChilde) return;
      store.addViews(
        uuid,
        firstChilde?.getBoundingClientRect() as DOMRect,
        firstChilde as Element,
      );
    });

    onUnmounted(() => {
      store.removeViews(uuid);
    });

    return () => {
      return (
        <div
          ref={(el) => {
            wrapper.value = el as HTMLElement;
          }}
          class={s.page_container}
        >
          {(slots.default && slots.default()) || ""}
        </div>
      );
    };
  },
});
