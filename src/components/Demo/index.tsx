import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "ProjectDemo",
  setup() {
    const greetMsg = ref("");
    const name = ref("");

    const handleGreet = async () => {
      greetMsg.value = await invoke("greet", {
        name: name.value,
      });
    };

    return () => {
      return (
        <div>
          this is Demo,greetMsg is: {greetMsg.value}
          <button onClick={handleGreet}>greet</button>
        </div>
      );
    };
  },
});
