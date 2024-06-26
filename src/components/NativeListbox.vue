<script lang="ts">
export interface NativeListboxExposed {
  focus: () => void;
  select: (idx: number) => void;
  selectNext: () => void;
  selectPrev: () => void;
  deselect: () => void;
}
</script>

<script setup lang="ts" generic="T">
import { onMounted, onUnmounted, ref } from 'vue';

const props = defineProps<{
  items: T[],
}>();

defineSlots<{
  item(props: T): any,
}>();

const emit = defineEmits<{
  (e: "item:selected", item: T): void,
  (e: "item:dblclick", item: T): void,
  (e: "item:keydown", item: T): void,
}>();

onMounted(() => {
  window.addEventListener("mouseup", handleItemDragStop);
});

onUnmounted(() => {
  window.removeEventListener("mouseup", handleItemDragStop);
});

const listRef = ref<HTMLElement | null>(null);
const isItemDragging = ref(false);
const selectedItemIdx = defineModel<number | null>("index", { default: null });
const pickedItem = defineModel<T | null>();

function handleItemDoubleClick() {
  if (!pickedItem.value)
    return;

  emit("item:dblclick", pickedItem.value);
}

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
  if (props.items[selectedItemIdx.value] !== undefined) {
    emit('item:selected', props.items[selectedItemIdx.value]);
  }
}

function handleItemPick() {
  if (selectedItemIdx.value === null)
    return;

  pickedItem.value = props.items[selectedItemIdx.value];
  if (props.items[selectedItemIdx.value] !== undefined) {
    emit('item:selected', props.items[selectedItemIdx.value]);
    emit('item:keydown', props.items[selectedItemIdx.value]);
  }
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

function focus() {
  if (listRef.value)
    listRef.value.focus();
}

function select(idx: number) {
  selectedItemIdx.value = idx;
  if (idx < 0 || idx >= props.items.length)
    return;

  pickedItem.value = props.items[idx];
  emit('item:selected', props.items[idx]);
}

function selectNext() {
  if (selectedItemIdx.value === null) {
    selectedItemIdx.value = 0;
  } else if (selectedItemIdx.value + 1 < props.items.length) {
    selectedItemIdx.value++;
  }
  pickedItem.value = props.items[selectedItemIdx.value];
  emit('item:selected', props.items[selectedItemIdx.value]);
}

function selectPrev() {
  if (!selectedItemIdx.value) {
    selectedItemIdx.value = 0;
  } else {
    selectedItemIdx.value--;
  }
  pickedItem.value = props.items[selectedItemIdx.value];
  emit('item:selected', props.items[selectedItemIdx.value]);
}

function deselect() {
  selectedItemIdx.value = null;
  pickedItem.value = null;
}

defineExpose({
  focus,
  select,
  selectNext,
  selectPrev,
  deselect,
});
</script>

<template>
  <ul
    ref="listRef"
    class="listbox"
    tabindex="0"
    @mouseup="handleItemDragStop"
    @keydown.enter="handleItemPick"
    @keydown.down="handleItemNext"
    @keydown.up="handleItemPrev"
  >
    <template v-if="$slots.item" v-for="(item, idx) in items">
      <li
        :class="{
          'listbox__item': true,
          'listbox__item--selected': selectedItemIdx == idx
        }"
        @mousedown="(e) => handleItemDragStart(idx, e)"
        @mouseenter="handleItemDragUpdate(idx)"
        @dblclick="handleItemDoubleClick"
      >
        <slot name="item" v-bind="item" />
      </li>
    </template>
    <template v-else>
      <li
        v-for="(item, idx) in items"
        :class="{
          'listbox__item': true,
          'listbox__item--selected': selectedItemIdx == idx
        }"
        :key="idx"
        @mousedown="(e) => handleItemDragStart(idx, e)"
        @mouseenter="handleItemDragUpdate(idx)"
      >
        {{item}}
      </li>
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
  margin: 0;
  padding: 0px 5px;
  text-align: left;
  white-space: nowrap;
  @include user-select-none;
}

.listbox__item--selected {
  background-color: $selection-background-color;
}
</style>
