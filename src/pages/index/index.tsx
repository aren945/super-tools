import PageContainer from "@/components/PageContainer";
import { defineComponent, onMounted, ref } from "vue";
import s from "./index.module.less";
import { Input } from "ant-design-vue";
import { listen } from "@tauri-apps/api/event";
import { getWinodwHiddenEvent } from "@/utils/events";

export default defineComponent({
  name: "IndexPage",
  setup() {
    const searchVaule = ref();
    const tools = ref<string[]>([]);

    onMounted(() => {
      // window窗口隐藏后，清空搜索框的值
      listen(getWinodwHiddenEvent(), () => {
        searchVaule.value = "";
      });
    });

    const handleEnter = () => {
      if (!searchVaule.value) {
        tools.value = [];
        return;
      }
      tools.value.push(searchVaule.value);
    };

    return () => {
      return (
        <PageContainer>
          <div class={s.index_page} v-auto-window-size>
            <div class={s.main_input_wrapper}>
              <Input
                v-model:value={searchVaule.value}
                class={s.input}
                placeholder={"请输入"}
                onPressEnter={handleEnter}
              />
            </div>
            <div class={s.tools_wrapper}>
              {tools.value.map((tool) => {
                return <div class={s.tool}>{tool}</div>;
              })}
            </div>
          </div>
        </PageContainer>
      );
    };
  },
});
