import { createApp } from "vue";
import Unicon from "vue-unicons";
import {
  uniBold,
  uniItalic,
  uniUnderline,
  uniMicrophone,
  uniPaperclip,
  uniAngleDown,
  uniArrow,
// @ts-ignore 6133
} from "vue-unicons/dist/icons";
import customIcons from "utils/customIcons";

import AddWindow from "./AddWindow.vue";
import "styles/_global.scss";

Unicon.add([
  customIcons.customSettings,
  customIcons.customSuperscript,
  customIcons.customSubscript,
  customIcons.customFunction,
  customIcons.customPin,
  uniBold,
  uniItalic,
  uniUnderline,
  uniMicrophone,
  uniPaperclip,
  uniAngleDown,
  uniArrow,
]);

// @ts-ignore 2374
createApp(AddWindow).use(Unicon).mount("#app");
