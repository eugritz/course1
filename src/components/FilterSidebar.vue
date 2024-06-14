<script lang="ts">
export interface FilterSidebarExposed {
  reset: () => void;
}

interface FilterSidebarItem {
  level?: number;
  value: string;
  icon?: ItemIcons;
  subitems?: FilterSidebarItem[];
  payload?: unknown;
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
import { invoke } from '@tauri-apps/api';
import { Event as UiEvent } from '@tauri-apps/api/event';
import { showMenu } from 'ext/tauri-plugin-context-menu';

import { Deck } from 'entities/Deck';
import { EntryKind } from 'entities/EntryKind';

import { deckStore } from 'stores/deckStore';
import { entryKindStore } from 'stores/entryKindStore';
import { tagStore } from 'stores/tagStore';

import { useTauriEvent } from 'utils/tauriEvent';
import dataEvents from 'constants/dataEvents';
import uiEvents from 'constants/uiEvents';

import FilterSidebarItem, { ItemIcons } from './FilterSidebarItem.vue';
import NativeListbox, { NativeListboxExposed } from './NativeListbox.vue';

const emit = defineEmits<{
  (e: "filter", query: string): void,
}>();

const decks = computed(() => deckStore.cached_all);
const entryKinds = computed(() => entryKindStore.cached_all);
const tags = computed(() => tagStore.cached_all);

const searchQuery = ref("");
const selectedItem = shallowRef<FilterSidebarItem | null>(null);
const defaultItems = (): FilterSidebarItem[] => ([
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
    icon: "entryKind",
    value: "Виды записей",
    subitems: [],
  },
  {
    icon: "tag",
    value: "Метки",
    subitems: [
      // { icon: "tagUnspecified", value: "Без меток" },
    ],
  },
]);

const mapIdToItemIdx = {
  decks: 1,
  entryKinds: 4,
  tags: 5,
};

const items = ref<FilterSidebarItem[]>(defaultItems());
const flattenItems = computed<FilterSidebarItem[]>(() =>
  searchQuery.value === ""
  ? flatten(items.value)
  : flattenFind(searchQuery.value.trim().toLowerCase(), items.value));

const filterSidebarListboxRef = ref<NativeListboxExposed | null>(null);

useTauriEvent("menu:renameTag", (event: UiEvent<unknown>) => {
  const payload = event.payload as string;
  invoke(uiEvents.InputModal.open, {
    id: "renameTag",
    title: "Переименовать метку",
    label: "Новое название метки",
    value: payload,
    placeholder: payload,
    payload,
    buttonText: "Изменить",
  });
});

useTauriEvent("menu:deleteTag", (event: UiEvent<unknown>) => {
  const payload = event.payload as string;
  invoke(uiEvents.ConfirmationModal.open, {
    id: "deleteTag",
    title: "Удалить метку",
    message: "Вы уверены, что хотите удалить метку?",
    payload,
  });
});

useTauriEvent(uiEvents.ConfirmationModal.onResult, (event: UiEvent<unknown>) => {
  const payload = event.payload as {
    id: string,
    button: number,
    payload: string,
  };

  if (payload.button !== 1)
    return;

  switch (payload.id) {
    case "deleteTag":
      tagStore.delete(payload.payload);
      items.value[mapIdToItemIdx.tags].subitems =
        items.value[mapIdToItemIdx.tags].subitems?.filter(
          (item) => item.payload !== payload.payload,
        );
      break;
    default:
      break;
  }
});

useTauriEvent(uiEvents.InputModal.onResult, (event: UiEvent<unknown>) => {
  const payload = event.payload as {
    id: string,
    input: string,
    payload: string,
  };

  switch (payload.id) {
    case "renameTag":
      const new_name = payload.input
        .replace(/\s/, "_")
        .replace(/^_+/, "")
        .replace(/_+$/, "")
        .replace(/_{2,}/, "_")
        .toLowerCase();
      tagStore.rename(payload.payload, new_name);
      const item = items.value[mapIdToItemIdx.tags].subitems?.find(
        (item) => item.payload === payload.payload
      );
      if (item) {
        item.value = new_name;
        item.payload = new_name;
      }
      break;
    default:
      break;
  }
});

useTauriEvent(dataEvents.update.tags, () => {
  tagStore.all();
});

useTauriEvent(uiEvents.window_open, load);

onMounted(load);

watch(searchQuery, () => {
  filterSidebarListboxRef.value?.deselect();
});

watch(selectedItem, () => {
  switch (selectedItem.value?.icon) {
    case "deck":
      const deck = selectedItem.value.payload as Deck;
      emit("filter", "колода:\"" + deck.name + "\"");
      break;
    case "entryKind":
      if (selectedItem.value.subitems === undefined) {
        const entryKind = selectedItem.value.payload as EntryKind;
        emit("filter", "вид:\"" + entryKind.name + "\"");
      }
      break;
    case "tagUnspecified":
      emit("filter", "метка:\"\"");
      break;
    case "tag":
      if (selectedItem.value.subitems === undefined) {
        emit("filter", "метка:" + selectedItem.value.payload);
      }
      break;
    default:
      break;
  }
});

watch(decks, () => {
  items.value[mapIdToItemIdx.decks].subitems = [];
  for (let i = 0; i < decks.value.length; i++) {
    const deck = decks.value[i];
    items.value[mapIdToItemIdx.decks].subitems?.push({
      icon: "deck",
      value: deck.name,
      payload: deck,
    });
  }
});

watch(entryKinds, () => {
  items.value[mapIdToItemIdx.entryKinds].subitems = [];
  for (let i = 0; i < entryKinds.value.length; i++) {
    const entryKind = entryKinds.value[i];
    items.value[mapIdToItemIdx.entryKinds].subitems?.push({
      icon: "entryKind",
      value: entryKind.name,
      payload: entryKind,
    });
  }
});

watch(tags, () => {
  items.value[mapIdToItemIdx.tags].subitems = [];
  items.value[mapIdToItemIdx.tags].subitems?.push(
    { icon: "tagUnspecified", value: "Без меток" },
  );

  for (let i = 0; i < tags.value.length; i++) {
    const tag = tags.value[i];
    items.value[mapIdToItemIdx.tags].subitems?.push({
      icon: "tag",
      value: tag.name,
      payload: tag.name,
    });
  }
});

function reset() {
  searchQuery.value = "";
  filterSidebarListboxRef.value?.deselect();
}

function load() {
  items.value = defaultItems();
  deckStore.all();
  entryKindStore.all();
  tagStore.all();
}

function handleOpenContextMenu(event: Event, item: FilterSidebarItem) {
  event.preventDefault();

  switch (item.icon) {
    case "tag":
      showMenu({
        items: [
          {
            label: "Переименовать",
            event: "menu:renameTag",
            payload: item.payload,
          },
          {
            label: "Удалить",
            event: "menu:deleteTag",
            payload: item.payload,
          },
        ],
      });
      break;
    default:
      break;
  }
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
      <div class="filter-sidebar__list__wrapper">
        <NativeListbox
          v-model="selectedItem"
          ref="filterSidebarListboxRef"
          class="filter-sidebar__list"
          :items="flattenItems"
        >
          <template #item="slotProps">
            <FilterSidebarItem
              v-bind="slotProps"
              @contextmenu="handleOpenContextMenu($event, slotProps)"
            />
          </template>
        </NativeListbox>
      </div>
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

.filter-sidebar__list__wrapper {
  position: relative;
  height: 100%;
  overflow-x: hidden;
  overflow-y: auto;
}

.filter-sidebar__list {
  position: absolute;
}
</style>
