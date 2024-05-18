<script lang="ts">
interface FilterSidebarItem {
  level?: number;
  value: string;
  icon?: ItemIcons;
  subitems?: FilterSidebarItem[];
}

function flatten(
  items: FilterSidebarItem[], level: number
): FilterSidebarItem[] {
  let flat = <FilterSidebarItem[]>[];
  for (let i = 0; i < items.length; i++) {
    const item = items[i];

    const clone = JSON.parse(JSON.stringify(item)) as FilterSidebarItem;
    clone.level = level;

    flat.push(clone);
    if (item.subitems && item.subitems.length > 0) {
      flat = flat.concat(flatten(item.subitems, level + 1));
    }
  }

  return flat;
}
</script>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, shallowRef, watch } from 'vue';

import { deckStore } from 'stores/deckStore';

import NativeListbox from './NativeListbox.vue';
import FilterSidebarItem, { ItemIcons } from './FilterSidebarItem.vue';

const decks = computed(() => deckStore.cached);
const sidebarWidth = ref(300);
const isDraggableDragging = ref(false);

const selectedItem = shallowRef<any | null>(null);
const items = ref<FilterSidebarItem[]>([
  {
    icon: "today",
    value: "Сегодня",
    subitems: [
      { icon: "today", value: "В очереди" },
      { icon: "today", value: "Добавленные" },
      { icon: "today", value: "Измененные" },
      { icon: "today", value: "Изученные" },
    ],
  },
  {
    icon: "decks",
    value: "Колоды",
    subitems: [],
  },
  {
    icon: "flag",
    value: "Флажки",
    subitems: [
      { icon: "flagUnspecified", value: "Без флажка" },
      { icon: "flagRed", value: "Красный" },
      { icon: "flagOrange", value: "Оранжевый" },
      { icon: "flagGreen", value: "Зеленый" },
      { icon: "flagBlue", value: "Синий" },
      { icon: "flagPink", value: "Розовый" },
      { icon: "flagTorquoise", value: "Бирюзовый" },
      { icon: "flagPurple", value: "Фиолетовый" },
    ],
  },
  {
    icon: "cardState",
    value: "Состояния карт",
    subitems: [
      { icon: "cardStateNew", value: "Новые" },
      { icon: "cardStateLearning", value: "Изучаемые" },
      { icon: "cardStateSuspended", value: "Отложенные" },
    ],
  },
  {
    icon: "cardKind",
    value: "Виды карт",
    subitems: [],
  },
  {
    icon: "tag",
    value: "Метки",
    subitems: [
      { icon: "tagUnspecified", value: "Без меток" },
    ],
  },
]);

const flattenItems = computed<FilterSidebarItem[]>(() =>
  flatten(items.value, 1));

const mapIdToItemIdx = {
  decks: 1,
};

onMounted(() => {
  deckStore.all();

  window.addEventListener("mousemove", handleDraggableDragUpdate);
  window.addEventListener("mouseup", handleDraggableDragStop);
});

onUnmounted(() => {
  window.removeEventListener("mousemove", handleDraggableDragUpdate);
  window.removeEventListener("mouseup", handleDraggableDragStop);
});

watch(decks, () => {
  console.log("bruh");
  for (let i = 0; i < deckStore.cached.length; i++) {
    const deck = deckStore.cached[i];
    items.value[mapIdToItemIdx.decks].subitems?.push({
      icon: "deck",
      value: deck.name,
    });
  }
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
      <NativeListbox v-model="selectedItem" class="filter-sidebar__list" :items="flattenItems">
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
  height: 100%;
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
