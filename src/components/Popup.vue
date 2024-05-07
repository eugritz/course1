<script lang="ts">
export interface PopupExposed {
  toggle: (event: Event) => void,
}
</script>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, shallowRef, watch } from 'vue';
import { useWindowSize } from 'utils/windowSize';

const state = ref(false);
const classes = ref({
  "popup": true,
  "popup--left": true,
  "popup--right": false,
});
const popup = shallowRef<HTMLElement | null>(null);
const target = shallowRef<HTMLElement | null>(null);

const { width, height } = useWindowSize();

watch([width, height], () => {
  state.value = false;
});

const position = computed(() => {
  width.value;
  height.value;

  const rect1 = target.value?.getBoundingClientRect();
  const rect2 = popup.value?.getBoundingClientRect();

  let padding = 0;
  if (popup.value) {
    padding = parseFloat(getComputedStyle(popup.value).paddingRight);
  }

  let left;
  let top;
  if (rect1 && rect2) {
    left = rect1.left;
    top = rect1.top + rect1.height;

    if (left + rect2.width + 20 > width.value) {
      left = rect1.right - rect2.width + padding;
      popup.value?.classList.remove("popup--left");
      popup.value?.classList.add("popup--right");
      classes.value['popup--left'] = false;
      classes.value['popup--right'] = true;
    } else {
      popup.value?.classList.add("popup--left");
      popup.value?.classList.remove("popup--right");
      classes.value['popup--left'] = true;
      classes.value['popup--right'] = false;
    }

    left = left + "px";
    top = top + "px";
  }

  return {
    left,
    top,
  };
});

function handleClickOutside(e: MouseEvent) {
  const clickTarget = e.target as HTMLElement;
  if (!target.value?.contains(clickTarget) && !popup.value?.contains(clickTarget)) {
    state.value = false;
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.code === "Escape") {
    state.value = false;
    e.stopPropagation();
  }
}

onMounted(() => {
  document.addEventListener("keydown", handleKeyDown);
  document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("keydown", handleKeyDown);
  document.removeEventListener("click", handleClickOutside);
});

function toggle(event: Event) {
  const eventTarget = event.currentTarget as HTMLElement;
  state.value = !state.value;
  target.value = eventTarget;
}

defineExpose({
  toggle,
});
</script>

<template>
  <Teleport to="body" v-if="state">
    <div
      ref="popup"
      :class="{ ...classes }"
      :style="{ ...position }"
    >
      <slot />
    </div>
  </Teleport>
</template>

<style scoped lang="scss">
@import "../styles/theme";

$popup-background-color: #ffffff;
$popup-border-width: 1px;
$popup-border-color: #e2e2e2;

@if $theme == dark {
  $popup-background-color: #1b1b1b;
  $popup-border-width: 1px;
  $popup-border-color: #454545;
}

.popup,
.popup--left,
.popup--right {
  --arrow-height: 8px;
  --arrow-left: 6px;

  z-index: 999;
  position: absolute;
  margin-top: var(--arrow-height);
  border-width: $popup-border-width;
  border-style: solid;
  border-radius: 8px;
  transform: translateX(calc(-1 * var(--arrow-left)));
  padding: 10px;
  border-color: $popup-border-color;
  background-color: $popup-background-color;

  @if $theme == light {
    box-shadow: 0px 3px 5px $popup-border-color;
  }
}

@mixin down-arrow {
  content: " ";
  position: absolute;
  bottom: 100%;
  width: 0;
  height: 0;
  border-style: solid;
  border-color: rgba(0, 0, 0, 0);
  border-top-width: 0;
  pointer-events: none;
}

.popup--left::before {
  @include down-arrow;
  left: var(--arrow-left);
  border-width: calc(var(--arrow-height) + $popup-border-width + 1px);
  border-bottom-color: $popup-border-color;
}

.popup--left::after {
  @include down-arrow;
  left: calc(var(--arrow-left) + $popup-border-width + 1px);
  border-width: var(--arrow-height);
  border-bottom-color: $popup-background-color;
}

.popup--right::before {
  @include down-arrow;
  right: calc(var(--arrow-left));
  border-width: calc(var(--arrow-height) + $popup-border-width + 1px);
  border-bottom-color: $popup-border-color;
}

.popup--right::after {
  @include down-arrow;
  right: calc(var(--arrow-left) + $popup-border-width + 1px);
  border-width: var(--arrow-height);
  border-bottom-color: $popup-background-color;
}
</style>
