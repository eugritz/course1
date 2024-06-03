<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Event as UiEvent, TauriEvent } from '@tauri-apps/api/event';

import { entryStore } from 'stores/entryStore';
import { useTauriEvent } from 'utils/tauriEvent';

import CardSwitch from 'components/CardSwitch.vue';
import Column from 'components/Column.vue';
import DataTable, { DataTableExposed } from 'components/DataTable.vue';
import FilterSidebar, { FilterSidebarExposed } from 'components/FilterSidebar.vue';
import Splitter, { SplitterExposed } from 'components/Splitter.vue';
import SplitterPanel from 'components/SplitterPanel.vue';
import Editor from 'components/Editor.vue';

const filterSidebar = ref<FilterSidebarExposed | null>(null);
const splitter = ref<SplitterExposed | null>(null);
const dataTable = ref<DataTableExposed | null>(null);

const cardSwitch = ref(false);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);

onMounted(() => {
  reset();
});

function reset(event?: UiEvent<unknown>) {
  if (event && event.windowLabel !== "EntryWindow")
    return;

  filterSidebar.value?.reset();
  splitter.value?.reset();
  dataTable.value?.reset();

  cardSwitch.value = false;
}

function searchEntries(event: Event) {
  const target = event.target as HTMLInputElement;
  entryStore.filter(target.value);
}

function handleItemNext() {
  if (dataTable.value)
    dataTable.value.selectNext();
}

function handleItemPrev() {
  if (dataTable.value)
    dataTable.value.selectPrev();
}
</script>

<template>
  <Splitter class="content" ref="splitter">
    <SplitterPanel size="20%" min-size="200px">
      <FilterSidebar ref="filterSidebar" />
    </SplitterPanel>
    <SplitterPanel class="data-view" min-size="400px">
      <div class="data-view__controls">
        <CardSwitch class="data-view__controls__switch" v-model="cardSwitch" />
        <input
          class="data-view__controls__search"
          type="text"
          placeholder="Поиск по картам/записям"
          @keydown.enter="searchEntries"
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
            ref="dataTable"
            tabindex="-1"
            class="data-view__data__table"
            :value="[{ test: 'aaaaaaaaaaaaaaaa' }]"
          >
            <template v-if="!cardSwitch">
              <Column field="test" header="Основное поле" />
              <Column field="test" header="Карта" />
              <Column field="test" header="Появление" />
              <Column field="test" header="Колода" />
            </template>
            <template v-else>
              <Column field="test" header="Основное поле" />
              <Column field="test" header="Запись" />
              <Column field="test" header="Карты" />
              <Column field="test" header="Метки" />
              <Column field="test" header="Появление" />
              <Column field="test" header="Создание" />
            </template>
          </DataTable>
        </div>
      </div>
    </SplitterPanel>
    <SplitterPanel min-size="540px">
      <div class="editor__wrapper">
        <Editor />
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
.editor__wrapper {
  height: calc(100vh - 16px);

  button {
    box-shadow: none;
  }

  .editor__section__tags {
    input {
      box-shadow: none;
    }
  }
}
</style>
