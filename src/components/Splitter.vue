<script setup lang="ts">
import { computed, onMounted, ref, useSlots, watchEffect } from 'vue';
import Resizer from './Resizer.vue';

const slots = useSlots();
const panels = computed(() => slots && slots.default ? slots.default() : []);
const panelSizes = ref<number[]>([]);

const containerWidth = ref<number | null>(null);
const resizeStartX = ref<number | null>(null);
const resizeStartPanelIdx = ref<number | null>(null);
const resizeStartPanelWidth = ref<number | null>(null);

const newPrevSize = ref<number | null>(null);
const newNextSize = ref<number | null>(null);

const root = ref<HTMLElement | null>(null);
const children = computed<HTMLElement[]>(() => {
  if (!root.value)
    return [];

  let children = <HTMLElement[]>[];
  for (let i = 0; i < root.value.children.length; i++) {
    const child = root.value.children[i] as HTMLElement;
    if (!child.classList.contains("splitter__splitter-panel"))
      continue;
    children.push(child);
  }

  return children;
});

onMounted(() => {
  reset();
});

watchEffect(() => handleResizeStart);

function reset() {
  panelSizes.value = [];
  let leftPanelSize = 100;
  let leftPanelCount = panels.value.length;

  let containerWidth = null;
  if (root.value) {
    containerWidth = root.value.getBoundingClientRect().width;
  }

  for (let i = 0; i < panels.value.length; i++) {
    const panel = panels.value[i];

    if (panel.props && panel.props.size) {
      const size = panel.props.size as string;
      let relSize;

      if (containerWidth && size.endsWith("px")) {
        relSize = parseFloat(size) / containerWidth;
      } else {
        relSize = parseFloat(size);
      }

      panelSizes.value.push(relSize);
      leftPanelSize -= relSize;
      leftPanelCount -= 1;
    } else {
      panelSizes.value.push(0);
    }
  }

  let defaultPanelSize = leftPanelSize / leftPanelCount;
  for (let i = 0; i < panels.value.length; i++) {
    if (panelSizes.value[i] == 0) {
      panelSizes.value[i] = defaultPanelSize;
    }
  }

  if (!root.value)
    return;

  let panelIdx = 0;
  for (let i = 0; i < root.value.children.length; i++) {
    const child = root.value.children[i] as HTMLElement;
    if (!child.classList.contains("splitter__splitter-panel"))
      continue;

    child.style.flexBasis = `calc(${panelSizes.value[panelIdx]}%)`;
    panelIdx += 1;
  }
}

function handleResizeStart(panelIdx: number, event: MouseEvent) {
  if (!root.value)
    return;

  containerWidth.value = root.value.getBoundingClientRect().width;
  resizeStartX.value = event?.clientX;
  resizeStartPanelIdx.value = panelIdx;
  resizeStartPanelWidth.value =
    children.value[panelIdx].getBoundingClientRect().width;
}

function handleResize(event: MouseEvent) {
  if (containerWidth.value === null
    || resizeStartX.value === null
    || resizeStartPanelIdx.value === null
    || resizeStartPanelWidth.value === null)
    return;

  const panelIdx = resizeStartPanelIdx.value;

  const dx = event.clientX - resizeStartX.value;
  const oldSize = panelSizes.value[panelIdx];
  const _newPrevSize = (resizeStartPanelWidth.value + dx) / containerWidth.value * 100;
  const _newNextSize = oldSize + panelSizes.value[panelIdx + 1] - _newPrevSize;

  if (!validateResize(_newPrevSize, _newNextSize))
    return;

  children.value[panelIdx].style.flexBasis = `calc(${_newPrevSize}%)`;
  children.value[panelIdx + 1].style.flexBasis = `calc(${_newNextSize}%)`;
  newPrevSize.value = _newPrevSize;
  newNextSize.value = _newNextSize;
}

function handleResizeStop() {
  if (resizeStartPanelIdx.value === null
    || newPrevSize.value === null
    || newNextSize.value === null)
    return;

  const panelIdx = resizeStartPanelIdx.value;
  panelSizes.value[panelIdx] = newPrevSize.value;
  panelSizes.value[panelIdx + 1] = newNextSize.value;
}

function validateResize(newPrevSize: number, newNextSize: number): boolean {
  if (containerWidth.value === null
    || !resizeStartX.value
    || resizeStartPanelIdx.value === null
    || resizeStartPanelWidth.value === null)
    return false;

  if (newPrevSize < 0 || newNextSize < 0 || newPrevSize >= containerWidth.value)
    return false;

  const panelIdx = resizeStartPanelIdx.value;
  const panel = panels.value[panelIdx];

  if (panel.props && (panel.props.minSize || panel.props["min-size"])) {
    const minSize = panel.props.minSize || panel.props["min-size"] as string;

    if (minSize.endsWith("px")) {
      const absMinSize = parseFloat(minSize);
      if (newPrevSize / 100 * containerWidth.value < absMinSize)
        return false;
    } else {
      const relMinSize = parseFloat(minSize);
      if (newPrevSize < relMinSize)
        return false;
    }
  }

  return true;
}
</script>

<template>
  <div class="splitter" ref="root">
    <template v-for="(panel, idx) in panels">
      <div class="splitter__splitter-panel">
        <component :is="panel" />
      </div>
      <Resizer
        v-if="idx !== panels.length - 1"
        @resizestart="handleResizeStart(idx, $event)"
        @resize="handleResize"
        @resizeend="handleResizeStop"
      />
    </template>
  </div>
</template>

<style scoped lang="scss">
.splitter {
  display: flex;
}
</style>
