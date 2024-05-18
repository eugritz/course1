<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

const width = defineModel<number>();
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

  isDragging.value = true;
  document.body.style.cursor = "col-resize";
}

function handleDraggableDragUpdate(event: MouseEvent) {
  if (!isDragging.value)
    return;

  width.value = event.clientX;
  event.preventDefault();
  event.stopPropagation();
}

function handleDraggableDragStop() {
  isDragging.value = false;
  document.body.removeAttribute("style");
}
</script>

<template>
  <div
    v-bind="$attrs"
    class="draggable"
    @mousedown="handleDraggableDragStart"
  >
    <unicon class="icon" width="12" height="12" name="draggabledots"></unicon>
  </div>
  <Teleport to="body" v-if="isDragging">
    <div class="draggable-wrapper">
    </div>
  </Teleport>
</template>

<style lang="scss">
@import "../styles/theme";

.draggable {
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

.draggable-wrapper {
  z-index: 9999;
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}
</style>
