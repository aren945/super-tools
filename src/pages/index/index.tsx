import PageContainer from "@/components/PageContainer";
import { defineComponent } from "vue";
import s from "./index.module.less";

export default defineComponent({
  name: "IndexPage",
  setup() {
    return () => {
      return (
        <PageContainer>
          <div class={s.index_page}>
            <div class={s.main_input_wrapper}>
              <input class={s.input} placeholder="请输入" />
            </div>
          </div>
        </PageContainer>
      );
    };
  },
});
