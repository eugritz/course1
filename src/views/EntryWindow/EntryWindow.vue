<script lang="ts">
function compareNumbers(a: number, b: number, order: boolean): number {
  const order_ = order ? 1 : -1;
  return order_ * (a - b);
}

function compareStrings(a: string, b: string, order: boolean): number {
  const order_ = order ? 1 : -1;
  return order_ * ((a > b) ? 1 : ((a < b) ? -1 : 0));
}

function compareDates(a: Date | null, b: Date | null, order: boolean): number {
  if (a === null && b === null)
    return 0;
  else if (a === null && b !== null)
    return 1;
  else if (a !== null && b === null)
    return -1;

  const order_ = order ? 1 : -1;
  // @ts-ignore 18047
  return order_ * ((a > b) ? 1 : ((a < b) ? -1 : 0));
}
</script>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event as UiEvent, TauriEvent, emit } from '@tauri-apps/api/event';
import { showMenu } from 'ext/tauri-plugin-context-menu';

import { FilteredCard, FilteredEntry } from 'entities/Entry';
import { entryStore } from 'stores/entryStore';
import { useTauriEvent } from 'utils/tauriEvent';
import dataEvents from 'constants/dataEvents';
import uiEvents from 'constants/uiEvents';

import CardSwitch from 'components/CardSwitch.vue';
import Column from 'components/Column.vue';
import DataTable, { DataTableExposed } from 'components/DataTable.vue';
import FilterSidebar, { FilterSidebarExposed } from 'components/FilterSidebar.vue';
import Splitter, { SplitterExposed } from 'components/Splitter.vue';
import SplitterPanel from 'components/SplitterPanel.vue';
import Editor, { EditorExposed } from 'components/Editor.vue';
import LoadingBanner from 'components/LoadingBanner.vue';

const filterSidebarRef = ref<FilterSidebarExposed | null>(null);
const splitterRef = ref<SplitterExposed | null>(null);
const dataTableRef = ref<DataTableExposed | null>(null);
const editorRef = ref<EditorExposed | null>(null);

const query = ref("");
const cardSwitch = ref(false);
const switch_ = computed(() => cardSwitch.value ? "entries" : "cards");
const entries = ref<(FilteredEntry | FilteredCard)[]>([]);

const orderBy = ref<number | null>(null);
const order = ref<boolean>(false);
const selectedEntry = ref<FilteredEntry | FilteredCard | null>(null);

function isFilteredCards(_entries: (FilteredEntry | FilteredCard)[]):
  _entries is FilteredCard[]
{
  return !cardSwitch.value;
}

function isFilteredEntries(_entries: (FilteredEntry | FilteredCard)[]):
  _entries is FilteredEntry[]
{
  return cardSwitch.value;
}

const sorted = computed(() => {
  if (isFilteredCards(entries.value)) {
    switch (orderBy.value) {
      case 0:
        return entries.value.toSorted(
          (a, b) => compareStrings(a.sort_field, b.sort_field, order.value)
        );
      case 1:
        return entries.value.toSorted(
          (a, b) => compareStrings(a.card_name, b.card_name, order.value)
        );
      case 2:
        return entries.value.toSorted(
          (a, b) => compareDates(a.next_shown_at, b.next_shown_at, order.value)
        );
      case 3:
        return entries.value.toSorted(
          (a, b) => compareStrings(a.deck_name, b.deck_name, order.value)
        );
      default:
        return entries.value;
    }
  } else if (isFilteredEntries(entries.value)) {
    switch (orderBy.value) {
      case 0:
        return entries.value.toSorted(
          (a, b) => compareStrings(a.sort_field, b.sort_field, order.value)
        );
      case 1:
        return entries.value.toSorted(
          (a, b) => compareStrings(a.entry_kind_name, b.entry_kind_name, order.value)
        );
      case 2:
        return entries.value.toSorted(
          (a, b) => compareNumbers(a.card_count, b.card_count, order.value)
        );
      case 3:
        return entries.value.toSorted(
          (a, b) => compareStrings(a.joined_tags, b.joined_tags, order.value)
        );
      case 4:
        return entries.value.toSorted(
          (a, b) => compareDates(a.next_shown_at, b.next_shown_at, order.value)
        );
      case 5:
        return entries.value.toSorted(
          (a, b) => compareDates(a.created_at, b.created_at, order.value)
        );
      default:
        return entries.value;
    }
  } else {
    return entries.value;
  }
});

useTauriEvent("menu:deleteEntry", (event: UiEvent<unknown>) => {
  const payload = event.payload as string;
  invoke(uiEvents.ConfirmationModal.open, {
    id: "deleteEntry",
    title: "Удалить запись",
    message: "Вы уверены, что хотите удалить запись?",
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
    case "deleteEntry":
      const cancel = setTimeout(() => {
        entryStore.loading = true;
      }, 50);
      entryStore.delete(parseInt(payload.payload)).then(() => {
        emit(dataEvents.update.entry);
        clearTimeout(cancel);
        entryStore.loading = false;
      });
      break;
    default:
      break;
  }
});


useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(dataEvents.update.entry, load);
useTauriEvent(dataEvents.update.entryFieldValue, load);
useTauriEvent(dataEvents.update.tags, load);
useTauriEvent(uiEvents.window_open, load);

onMounted(() => {
  reset();
  load();
});

watch(cardSwitch, () => {
  entries.value = [];
  
  nextTick(() => {
    filterSidebarRef.value?.reset();
    dataTableRef.value?.reset();
    load();
  });
});

function reset(event?: UiEvent<unknown>) {
  if (event && event.windowLabel !== "EntryWindow")
    return;

  filterSidebarRef.value?.reset();
  splitterRef.value?.reset();
  dataTableRef.value?.reset();
  editorRef.value?.reset();

  query.value = "";
  cardSwitch.value = false;
  selectedEntry.value = null;
}

function load() {
  const isLoading = entryStore.loading;
  let cancel: Timer | null = null;
  if (!isLoading) {
    cancel = setTimeout(() => {
      entryStore.loading = true;
    }, 50);
  }

  entryStore.filter(switch_.value, query.value).then((entries_) => {
    entries.value = entries_;
    if (!isLoading) {
      clearTimeout(cancel!);
      entryStore.loading = false;
    }
  });
}

function handleFilter(query_: string) {
  if (query.value !== query_) {
    query.value = query_;
    load();
  }
}

function handleSearchEntries() {
  load();
}

function handleItemNext() {
  if (dataTableRef.value)
    dataTableRef.value.selectNext();
}

function handleItemPrev() {
  if (dataTableRef.value)
    dataTableRef.value.selectPrev();
}

function handleItemContextMenu(
  event: MouseEvent,
  item: FilteredEntry | FilteredCard
) {
  event.preventDefault();
  showMenu({
    items: [
      {
        label: "Удалить",
        event: "menu:deleteEntry",
        payload: item.id.toString(),
      },
    ],
  });
}
</script>

<template>
  <Splitter class="content" ref="splitterRef">
    <SplitterPanel size="20%" min-size="200px">
      <FilterSidebar ref="filterSidebarRef" @filter="handleFilter" />
    </SplitterPanel>
    <SplitterPanel class="data-view" min-size="400px">
      <div class="data-view__controls">
        <CardSwitch class="data-view__controls__switch" v-model="cardSwitch" />
        <input
          class="data-view__controls__search"
          type="text"
          placeholder="Нажмите Enter для поиска по картам/записям"
          v-model="query"
          @keydown.enter="handleSearchEntries"
        />
      </div>
      <div class="data-view__data">
        <div
          class="data-view__data__wrapper"
          tabindex="0"
          @keydown.down="handleItemNext"
          @keydown.up="handleItemPrev"
        >
          <DataTable
            ref="dataTableRef"
            tabindex="-1"
            class="data-view__data__table"
            v-model="selectedEntry"
            v-model:order="order"
            v-model:orderby="orderBy"
            :value="sorted"
            @item:contextmenu="handleItemContextMenu"
          >
            <template v-if="!cardSwitch">
              <Column field="sort_field" header="Основное поле" />
              <Column field="card_name" header="Карта" />
              <Column field="next_shown_at" header="Появление" />
              <Column field="deck_name" header="Колода" />
            </template>
            <template v-else>
              <Column field="sort_field" header="Основное поле" />
              <Column field="entry_kind_name" header="Вид" />
              <Column field="card_count" header="Карты" />
              <Column field="joined_tags" header="Метки" />
              <Column field="next_shown_at" header="Появление" />
              <Column field="created_at" header="Создание" />
            </template>
          </DataTable>
        </div>
      </div>
    </SplitterPanel>
    <SplitterPanel min-size="540px">
      <div class="editor__wrapper">
        <Editor ref="editorRef" :entry-id="selectedEntry?.id" />
      </div>
    </SplitterPanel>
  </Splitter>
  <LoadingBanner v-model="entryStore.loading" />
</template>

<style scoped lang="scss">
@import "../../styles/mixins";
@import "../../styles/theme";

.content {
  height: 100vh;
  justify-content: start;
}

.data-view {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.data-view__controls {
  display: flex;
  flex-direction: row;
  gap: 8px;
}

.data-view__controls__switch {
  flex-shrink: 0;

  @if $theme == dark {
    box-shadow: none;
  }
}

.data-view__controls__search {
  width: 100%;
  padding: 0.3em 0.6em;

  @if $theme == dark {
    box-shadow: none;
  }
}

.data-view__data {
  position: relative;
  display: flex;
  height: 100%;
}

.data-view__data__table {
  outline: none;
}

.data-view__data__wrapper {
  position: absolute;
  overflow: auto;
  width: 100%;
  height: calc(100% - 16px);
  border-radius: 4px;
  background-color: #272727;
  @include user-select-none;
}
</style>

<style lang="scss">
@import "../../styles/mixins";

body {
  @include user-select-none;
}

.editor__wrapper {
  height: calc(100vh - 16px);

  button {
    box-shadow: none;
  }

  .tags-input {
    box-shadow: none;
  }
}
</style>
