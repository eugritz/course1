import { createApp } from "vue";
import Unicon from "vue-unicons";
import {
  uniBook,
  uniBookAlt,
  uniBookOpen,
  uniCircle,
  uniClock,
  uniDraggabledots,
  uniTable,
  uniTagAlt,
// @ts-ignore 6133
} from "vue-unicons/dist/icons";
import customIcons from "utils/customIcons";

import CardsWindow from "./CardsWindow.vue";
import "styles/_global.scss";

Unicon.add([
  customIcons.customFlag,
  customIcons.customFlagUnspecified,
  customIcons.customFlagRed,
  customIcons.customFlagOrange,
  customIcons.customFlagGreen,
  customIcons.customFlagBlue,
  customIcons.customFlagTorquoise,
  customIcons.customFlagPink,
  customIcons.customFlagPurple,
  customIcons.customCardStateNew,
  customIcons.customCardStateLearning,
  customIcons.customCardStateSuspended,
  customIcons.customTagUnspecified,
  uniBook,
  uniBookAlt,
  uniBookOpen,
  uniCircle,
  uniClock,
  uniDraggabledots,
  uniTable,
  uniTagAlt,
])

// @ts-ignore 2374
createApp(CardsWindow).use(Unicon).mount("#app");
