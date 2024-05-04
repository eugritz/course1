<script setup lang="ts">
const emit = defineEmits<{
  (e: "click", event: MouseEvent | KeyboardEvent): void,
}>();

let isPressed = false;

function click(event: MouseEvent | KeyboardEvent) {
  emit("click", event);
}

function activate(event: Event) {
  if (event.target) {
    isPressed = true;
  }
}

function postActivate(event: Event) {
  if (event.target && isPressed) {
    isPressed = false;
    emit("click", event as KeyboardEvent);
  }
}
</script>

<template>
  <a
    class="link-button"
    tabindex="0"
    @click="click"
    @keydown.enter="click"
    @keydown.space="activate"
    @keyup.space="postActivate"
  >
    <slot></slot>
  </a>
</template>

<style scoped lang="scss">
@import "../styles/mixins";

.link-button {
  text-align: center;
  @include user-select-none;
  cursor: pointer;
}
</style>
