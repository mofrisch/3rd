import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { attachConsole } from "tauri-plugin-log-api";

attachConsole();
createApp(App).mount("#app");
