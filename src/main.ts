import { createApp } from "vue";
import Unicon from "vue-unicons";
// @ts-ignore 6133
import { uniEllipsisH } from "vue-unicons/dist/icons";

import App from "App.vue";
import "styles/_global.scss";

Unicon.add([uniEllipsisH])

// @ts-ignore 2374
createApp(App).use(Unicon).mount("#app");
