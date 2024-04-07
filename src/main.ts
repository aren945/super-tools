import { createApp } from "vue";
import App from "./App";
import router from "./router/index";
import "./style/reset.less";
import { globalEventPlugin } from "./global-event";
import { createPinia } from "pinia";
import Antd from "ant-design-vue";

const pinia = createPinia();

const app = createApp(App);

app.use(router).use(Antd).use(pinia).use(globalEventPlugin()).mount("#app");
