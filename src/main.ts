import { createApp } from "vue";
import App from "./App.vue";
import { router } from "./router";
import "./assets/tailwind.css"
import cookies from 'vue-cookies';

export const app = createApp(App).use(router).use(cookies).mount("#app");
