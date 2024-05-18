<script lang="ts">
export interface PopupExposed {
  toggle: (event: Event) => void;
  close: () => void;
}
</script>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, shallowRef, watch } from 'vue';
import { useWindowSize } from 'utils/windowSize';

const props = defineProps<{
  arrow?: boolean,
}>();

const classes = ref({
  "popup": true,
  "popup--right": true,
  "popup--left": false,
});
const styles = ref({
  "--arrow-height": props.arrow ? "8px" : "4px",
});

const state = ref(false);
const popup = shallowRef<HTMLElement | null>(null);
const target = shallowRef<HTMLElement | null>(null);

const { width, height } = useWindowSize();

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
      popup.value?.classList.remove("popup--right");
      popup.value?.classList.add("popup--left");
    } else {
      if (!props.arrow) {
        left += padding
      }

      popup.value?.classList.add("popup--right");
      popup.value?.classList.remove("popup--left");
    }

    left = left + "px";
    top = top + "px";
  }

  return {
    left,
    top,
  };
});

watch([width, height], () => {
  state.value = false;
});

onMounted(() => {
  document.addEventListener("keydown", handleKeyDown);
  document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("keydown", handleKeyDown);
  document.removeEventListener("click", handleClickOutside);
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

function toggle(event: Event) {
  const eventTarget = event.currentTarget as HTMLElement;
  state.value = !state.value;
  target.value = eventTarget;
}

function close() {
  state.value = !state.value;
}

defineExpose({
  toggle,
  close,
});
</script>

<template>
  <Teleport to="body" v-if="state">
    <div
      ref="popup"
      v-bind="$attrs"
      :class="{ ...classes }"
      :style="{ ...styles, ...position }"
    >
      <span v-if="$props.arrow" class="arrow"></span>
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
.popup--right,
.popup--left {
  --arrow-left: 8px;

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

.popup--left {
  .arrow::before {
    @include down-arrow;
    right: var(--arrow-left);
    border-width: calc(var(--arrow-height) + $popup-border-width + 1px);
    border-bottom-color: $popup-border-color;
  }
  
  .arrow::after {
    @include down-arrow;
    right: calc(var(--arrow-left) + $popup-border-width + 1px);
    border-width: var(--arrow-height);
    border-bottom-color: $popup-background-color;
  }
}

.popup--right {
  .arrow::before {
    @include down-arrow;
    left: var(--arrow-left);
    border-width: calc(var(--arrow-height) + $popup-border-width + 1px);
    border-bottom-color: $popup-border-color;
  }
  
  .arrow::after {
    @include down-arrow;
    left: calc(var(--arrow-left) + $popup-border-width + 1px);
    border-width: var(--arrow-height);
    border-bottom-color: $popup-background-color;
  }
}
</style>
