<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { Event as UiEvent, TauriEvent } from '@tauri-apps/api/event';

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

const filterSidebarRef = ref<FilterSidebarExposed | null>(null);
const splitterRef = ref<SplitterExposed | null>(null);
const dataTableRef = ref<DataTableExposed | null>(null);
const editorRef = ref<EditorExposed | null>(null);

const query = ref("");
const cardSwitch = ref(false);
const switch_ = computed(() => cardSwitch.value ? "entries" : "cards");
const entries = ref<(FilteredEntry | FilteredCard)[]>([]);

const selectedEntry = ref<FilteredEntry | FilteredCard | null>(null);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(dataEvents.update.entry, load);
useTauriEvent(dataEvents.update.entryFieldValue, load);
useTauriEvent(uiEvents.window_open, load);

onMounted(() => {
  reset();
  load();
});

watch(cardSwitch, () => {
  filterSidebarRef.value?.reset();
  dataTableRef.value?.reset();
  query.value = "";

  entryStore.filter(switch_.value).then((entries_) => {
    entries.value = entries_;
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
  entryStore.filter(switch_.value, query.value).then((entries_) => {
    entries.value = entries_;
  });
}

function handleFilter(query_: string) {
  query.value = query_;
  entryStore.filter(switch_.value, query.value).then((entries_) => {
    entries.value = entries_;
  });
}

function handleSearchEntries() {
  entryStore.filter(switch_.value, query.value).then((entries_) => {
    entries.value = entries_;
  });
}

function handleItemNext() {
  if (dataTableRef.value)
    dataTableRef.value.selectNext();
}

function handleItemPrev() {
  if (dataTableRef.value)
    dataTableRef.value.selectPrev();
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
            :value="entries"
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
