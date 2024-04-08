import { SlotsType, defineComponent, ref } from "vue";
import s from "./index.module.less";
import { JSX } from "vue/jsx-runtime";

export default defineComponent({
  name: "PageContainer",
  slots: {} as SlotsType<{
    default: () => JSX.Element;
  }>,
  setup(_, { slots }) {
    const wrapper = ref<Element>();
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
