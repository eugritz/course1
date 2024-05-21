<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

defineProps({
  icon: {
    type: Boolean,
    default: true,
  },
});

const emit = defineEmits<{
  (e: "resizestart", event: MouseEvent): void;
  (e: "resize", event: MouseEvent, start: MouseEvent): void;
  (e: "resizeend", event: MouseEvent, start: MouseEvent): void;
}>();

const resizeStartEvent = ref<MouseEvent | null>(null);
const isResizing = ref(false);

onMounted(() => {
  window.addEventListener("mousemove", handleResize);
  window.addEventListener("mouseup", handleResizeStop);
});

onUnmounted(() => {
  window.removeEventListener("mousemove", handleResize);
  window.removeEventListener("mouseup", handleResizeStop);
});

function handleResizeStart(event: MouseEvent) {
  if (event.button !== 0)
    return;

  emit("resizestart", event);
  isResizing.value = true;
  resizeStartEvent.value = event;
  document.body.style.cursor = "col-resize";
}

function handleResize(event: MouseEvent) {
  if (!isResizing.value || !resizeStartEvent.value)
    return;

  emit("resize", event, resizeStartEvent.value);
  event.preventDefault();
  event.stopPropagation();
}

function handleResizeStop(event: MouseEvent) {
  if (!isResizing.value || !resizeStartEvent.value)
    return;

  emit("resizeend", event, resizeStartEvent.value);
  isResizing.value = false;
  resizeStartEvent.value = null;
  document.body.removeAttribute("style");
}
</script>

<template>
  <div
    v-bind="$attrs"
    :class="{ resizer: icon, 'resizer--small': !icon }"
    @mousedown="handleResizeStart"
  >
    <unicon
      v-if="icon"
      class="icon"
      width="12"
      height="12"
      name="draggabledots"
    />
  </div>
  <Teleport to="body" v-if="isResizing">
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

.resizer--small {
  z-index: 2;
  position: absolute;
  top: 0;
  right: 0;
  width: 7px;
  margin-right: -5px;
  cursor: col-resize;
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
