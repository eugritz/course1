<script setup lang="ts">
import { onMounted, onUnmounted, ref, shallowRef, watch } from 'vue';

import NativeListbox from './NativeListbox.vue';
import FilterSidebarItem, { ItemIcons } from './FilterSidebarItem.vue';

const sidebarWidth = ref(300);
const isDraggableDragging = ref(false);

const selectedItem = shallowRef<any | null>(null);
const items = ref<{
  level?: number,
  value: string,
  icon?: ItemIcons,
}[]>([
  { value: "Сегодня", icon: "today" },
  { level: 2, value: "В очереди", icon: "today" },
  { level: 2, value: "Добавленные", icon: "today" },
  { level: 2, value: "Измененные", icon: "today" },
  { level: 2, value: "Изученные", icon: "today" },
  { value: "Колоды", icon: "decks" },
  { level: 2, value: "По умолчанию", icon: "deck" },
  { value: "Флажки", icon: "flag" },
  { level: 2, value: "Без флажка", icon: "flagUnspecified" },
  { level: 2, value: "Красный", icon: "flagRed" },
  { level: 2, value: "Оранжевый", icon: "flagOrange" },
  { level: 2, value: "Зеленый", icon: "flagGreen" },
  { level: 2, value: "Синий", icon: "flagBlue" },
  { level: 2, value: "Розовый", icon: "flagPink" },
  { level: 2, value: "Бирюзовый", icon: "flagTorquoise" },
  { level: 2, value: "Фиолетовый", icon: "flagPurple" },
  { value: "Состояния карт", icon: "cardState" },
  { level: 2, value: "Новые", icon: "cardStateNew" },
  { level: 2, value: "Изучаемые", icon: "cardStateLearning" },
  { level: 2, value: "Отложенные", icon: "cardStateSuspended" },
  { value: "Виды карт", icon: "cardKind" },
  { value: "Метки", icon: "tag" },
  { level: 2, value: "Без меток", icon: "tagUnspecified" },
]);

watch(selectedItem, () => {
  console.log(selectedItem.value);
});

onMounted(() => {
  window.addEventListener("mousemove", handleDraggableDragUpdate);
  window.addEventListener("mouseup", handleDraggableDragStop);
});

onUnmounted(() => {
  window.removeEventListener("mousemove", handleDraggableDragUpdate);
  window.removeEventListener("mouseup", handleDraggableDragStop);
});

function handleDraggableDragStart() {
  isDraggableDragging.value = true;
  document.body.style.cursor = "col-resize";
}

function handleDraggableDragUpdate(event: MouseEvent) {
  if (!isDraggableDragging.value)
    return;

  sidebarWidth.value = event.clientX;
}

function handleDraggableDragStop() {
  isDraggableDragging.value = false;
  document.body.removeAttribute("style");
}
</script>

<template>
  <div class="filter-sidebar" :style="{ width: sidebarWidth + 'px' }">
    <div class="filter-sidebar__content">
      <input
        class="filter-sidebar__search"
        type="text"
        placeholder="Фильтрация категорий"
      />
      <NativeListbox v-model="selectedItem" class="filter-sidebar__list" :items="items">
        <template #item="slotProps">
          <FilterSidebarItem v-bind="slotProps" />
        </template>
      </NativeListbox>
    </div>
    <div
      class="filter-sidebar__draggable"
      @mousedown="handleDraggableDragStart"
    >
      <unicon class="icon" width="12" height="12" name="draggabledots"></unicon>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/theme";

$draggable-width: 12px;

.filter-sidebar {
  position: absolute;
  height: calc(100% - 16px);
  display: flex;
  flex-direction: row;
}

.filter-sidebar__search {
  width: 100%;
  min-width: 200px;
  padding: 0.3em 0.6em;

  @if $theme == dark {
    box-shadow: none;
  }
}

.filter-sidebar__content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.filter-sidebar__list {
  overflow: auto;
}

.filter-sidebar__draggable {
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
</style>
