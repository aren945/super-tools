import { createApp } from "vue";
import App from "./App";
import router from "./router/index";
import "./style/reset.less";
import { globalEventPlugin } from "./global-event";
import { createPinia } from "pinia";

const pinia = createPinia();

const app = createApp(App);

app.use(router).use(pinia).use(globalEventPlugin()).mount("#app");
