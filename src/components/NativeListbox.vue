<script lang="ts">
export interface NativeListboxExposed {
  deselect: () => void;
}
</script>

<script setup lang="ts" generic="T">
import { onMounted, onUnmounted, ref, shallowRef } from 'vue';

const props = defineProps<{
  items: T[],
}>();

defineSlots<{
  item(props: T): any,
}>();

onMounted(() => {
  window.addEventListener("mouseup", handleItemDragStop);
});

onUnmounted(() => {
  window.removeEventListener("mouseup", handleItemDragStop);
});

const isItemDragging = ref(false);
const selectedItemIdx = shallowRef<number | null>(null);
const pickedItem = defineModel<T | null>();

function handleItemDragStart(item: number, event: MouseEvent) {
  if (event.button === 0)
    isItemDragging.value = true;
  selectedItemIdx.value = item;
}

function handleItemDragUpdate(item: number) {
  if (!isItemDragging.value)
    return;

  selectedItemIdx.value = item;
}

function handleItemDragStop() {
  if (selectedItemIdx.value === null)
    return;

  isItemDragging.value = false;
  pickedItem.value = props.items[selectedItemIdx.value];
}

function handleItemPick() {
  if (selectedItemIdx.value === null)
    return;

  pickedItem.value = props.items[selectedItemIdx.value];
}

function handleItemNext(_event: KeyboardEvent) {
  if (selectedItemIdx.value === null) {
    selectedItemIdx.value = 0;
  } else if (selectedItemIdx.value + 1 < props.items.length) {
    selectedItemIdx.value++;
  }
}

function handleItemPrev(_event: KeyboardEvent) {
  if (!selectedItemIdx.value) {
    selectedItemIdx.value = 0;
  } else {
    selectedItemIdx.value--;
  }
}

function deselect() {
  selectedItemIdx.value = null;
  pickedItem.value = null;
}

defineExpose({
  deselect,
});
</script>

<template>
  <ul
    class="listbox"
    tabindex="0"
    @mouseup="handleItemDragStop"
    @keydown.enter="handleItemPick"
    @keydown.down="handleItemNext"
    @keydown.up="handleItemPrev"
  >
    <template v-if="$slots.item" v-for="(item, idx) in items">
      <div
        :class="{
          'listbox__item': true,
          'listbox__item--selected': selectedItemIdx == idx
        }"
        @mousedown="(e) => handleItemDragStart(idx, e)"
        @mouseenter="handleItemDragUpdate(idx)"
      >
        <slot name="item" v-bind="item" />
      </div>
    </template>
    <template v-else>
      <div
        v-for="(item, idx) in items"
        class="item"
        :key="idx"
        :class="{ selected: selectedItemIdx == idx }"
        @mousedown="(e) => handleItemDragStart(idx, e)"
        @mouseenter="handleItemDragUpdate(idx)"
      >
        {{item}}
      </div>
    </template>
  </ul>
</template>

<style scoped lang="scss">
@import "../styles/mixins";
@import "../styles/variables";

.listbox {
  list-style: none;
}

.listbox__item {
  @include user-select-none;
}

.listbox__item--selected {
  background-color: $selection-background-color;
}
</style>
