import { SlotsType, defineComponent } from "vue";
import s from "./index.module.less";
import { JSX } from "vue/jsx-runtime";

export default defineComponent({
  name: "PageContainer",
  slots: {} as SlotsType<{
    default: () => JSX.Element;
  }>,
  setup(_, { slots }) {
    return () => {
      return (
        <div class={s.page_container}>
          {(slots.default && slots.default()) || ""}
        </div>
      );
    };
  },
});
