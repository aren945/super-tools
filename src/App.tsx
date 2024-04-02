import { defineComponent, onMounted } from "vue";
import { RouterView } from "vue-router";

export default defineComponent({
  name: "App",
  setup() {
    onMounted(() => {
      document.addEventListener("mousemove", (e) => {
        console.log(e);
      });
    });
    return () => {
      return (
        <div>
          <RouterView />
        </div>
      );
    };
  },
});
