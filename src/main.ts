import { createApp } from "vue";
import App from "./App";
import router from "./router/index";
import "./style/reset.less";

const app = createApp(App);

app.use(router).mount("#app");
