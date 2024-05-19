<script lang="ts">
export interface FilterSidebarExposed {
  reset: () => void;
}

interface FilterSidebarItem {
  level?: number;
  value: string;
  icon?: ItemIcons;
  subitems?: FilterSidebarItem[];
}

function flatten(
  items: FilterSidebarItem[],
  level: number = 1,
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

function flattenFind(
  query: string,
  items: FilterSidebarItem[],
  level: number = 1,
): FilterSidebarItem[] {
  let flat = <FilterSidebarItem[]>[];
  for (let i = 0; i < items.length; i++) {
    const item = items[i];

    if (item.subitems && item.subitems.length > 0) {
      if (item.subitems && item.subitems.length > 0) {
        const children = flattenFind(query, item.subitems, level + 1);
        if (children.length > 0) {
          const clone = JSON.parse(JSON.stringify(item)) as FilterSidebarItem;
          clone.level = level;

          flat.push(clone);
          flat = flat.concat(flattenFind(query, item.subitems, level + 1));
        }
      }
    } else {
      if (item.value.trim().toLowerCase().indexOf(query) !== -1) {
        const clone = JSON.parse(JSON.stringify(item)) as FilterSidebarItem;
        clone.level = level;
        flat.push(clone);
      }
    }
  }

  return flat;
}
</script>

<script setup lang="ts">
import { computed, onMounted, ref, shallowRef, watch } from 'vue';

import { deckStore } from 'stores/deckStore';

import FilterSidebarItem, { ItemIcons } from './FilterSidebarItem.vue';
import NativeListbox, { NativeListboxExposed } from './NativeListbox.vue';

const decks = computed(() => deckStore.cached);
const searchQuery = ref("");
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
  searchQuery.value === ""
  ? flatten(items.value)
  : flattenFind(searchQuery.value.trim().toLowerCase(), items.value));

const filterSidebarListbox = ref<NativeListboxExposed | null>(null);

const mapIdToItemIdx = {
  decks: 1,
};

onMounted(() => {
  deckStore.all();
});

watch(searchQuery, () => {
  filterSidebarListbox.value?.deselect();
});

watch(decks, () => {
  for (let i = 0; i < deckStore.cached.length; i++) {
    const deck = deckStore.cached[i];
    items.value[mapIdToItemIdx.decks].subitems?.push({
      icon: "deck",
      value: deck.name,
    });
  }
});

function reset() {
  searchQuery.value = "";
  filterSidebarListbox.value?.deselect();
}

defineExpose({
  reset,
});
</script>

<template>
  <div class="filter-sidebar">
    <div class="filter-sidebar__content">
      <input
        v-model="searchQuery"
        class="filter-sidebar__search"
        type="text"
        placeholder="Фильтрация категорий"
      />
      <NativeListbox
        v-model="selectedItem"
        ref="filterSidebarListbox"
        class="filter-sidebar__list"
        :items="flattenItems"
      >
        <template #item="slotProps">
          <FilterSidebarItem v-bind="slotProps" />
        </template>
      </NativeListbox>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/theme";

.filter-sidebar {
  min-width: 200px;
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
</style>
