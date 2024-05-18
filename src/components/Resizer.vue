<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

const emit = defineEmits<{
  (e: "resizestart", event: MouseEvent): void;
  (e: "resize", event: MouseEvent): void;
  (e: "resizeend", event: MouseEvent): void;
}>();

const isDragging = ref(false);

onMounted(() => {
  window.addEventListener("mousemove", handleDraggableDragUpdate);
  window.addEventListener("mouseup", handleDraggableDragStop);
});

onUnmounted(() => {
  window.removeEventListener("mousemove", handleDraggableDragUpdate);
  window.removeEventListener("mouseup", handleDraggableDragStop);
});

function handleDraggableDragStart(event: MouseEvent) {
  if (event.button !== 0)
    return;

  emit("resizestart", event);
  isDragging.value = true;
  document.body.style.cursor = "col-resize";
}

function handleDraggableDragUpdate(event: MouseEvent) {
  if (!isDragging.value)
    return;

  emit("resize", event);
  event.preventDefault();
  event.stopPropagation();
}

function handleDraggableDragStop(event: MouseEvent) {
  if (!isDragging.value)
    return;

  emit("resizeend", event);
  isDragging.value = false;
  document.body.removeAttribute("style");
}
</script>

<template>
  <div
    v-bind="$attrs"
    class="resizer"
    @mousedown="handleDraggableDragStart"
  >
    <unicon class="icon" width="12" height="12" name="draggabledots"></unicon>
  </div>
  <Teleport to="body" v-if="isDragging">
    <div class="resizer-wrapper">
    </div>
  </Teleport>
</template>

<style lang="scss">
@import "../styles/theme";

.resizer {
  flex: 0;
  display: flex;
  align-items: center;
  cursor: col-resize;

  .icon {
    fill: #848484;
  }

  @if $theme == dark {
    .icon {
      fill: #7f7f7f;
    }
  }
}

.resizer-wrapper {
  z-index: 9999;
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}
</style>
