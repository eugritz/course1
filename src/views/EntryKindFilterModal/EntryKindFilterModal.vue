<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, TauriEvent, emit } from '@tauri-apps/api/event';

import { EntryKind } from 'entities/EntryKind';
import { entryKindStore } from 'stores/entryKindStore';
import { useTauriEvent } from 'utils/tauriEvent';
import events from 'constants/events';

import NativeListbox, { NativeListboxExposed } from 'components/NativeListbox.vue';

const filter = ref("");
const filterRef = ref<HTMLElement | null>(null);
const listRef = ref<NativeListboxExposed | null>(null);

const selectedEntryKind = ref<EntryKind | null>(null);
const selectedEntryKindIdx = ref<number | null>(null);

const entryKinds = computed(() =>
  entryKindStore.cached_all.filter(
    (entryKind) =>
      entryKind.name.toLowerCase().includes(filter.value.trim().toLowerCase())));

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(events.EntryKindFilterModal.setData, handleSetData);
useTauriEvent(events.window_open, load);

onMounted(() => {
  load();
  reset();
});

function load() {
  entryKindStore.all();
}

function reset(event?: Event<unknown>) {
  if (event && event.windowLabel !== "EntryKindFilterModal")
    return;

  filter.value = "";
  if (filterRef.value)
    filterRef.value.focus();
  if (listRef.value)
    listRef.value.deselect();
}

function openEntryKindListModal() {
  emit(events.EntryKindListModal.setData, {
    selectedEntryKindId: selectedEntryKind.value?.id,
  }).then(() => {
    invoke(events.EntryKindListModal.open);
  })
}

function handleSetData(event: Event<unknown>) {
  const payload = event.payload as {
    selectedEntryKindId: number,
  };

  let found = entryKinds.value.findIndex((deck) =>
    deck.id == payload.selectedEntryKindId);
  selectedEntryKindIdx.value = found < 0 ? null : found;
}

function handleItemSelect(item: EntryKind) {
  emit(events.EntryKindFilterModal.onResult, {
    selectedEntryKindId: item.id,
  }).then(() => {
    invoke(events.window_close).then(() => {
      reset();
    });
  });
}

function handleSubmit() {
  if (!selectedEntryKind.value)
    return;

  handleItemSelect(selectedEntryKind.value);
}

function handleCancel() {
  invoke(events.window_close).then(() => {
    reset();
  });
}
</script>

<template>
  <div class="content" @keydown.esc="handleCancel">
    <div class="filter">
      Фильтр:
      <input type="text" ref="filterRef" v-model="filter"></input>
    </div>
    <div class="wrapper">
      <NativeListbox
        v-model="selectedEntryKind"
        v-model:index="selectedEntryKindIdx"
        ref="listRef"
        class="list"
        :items="entryKinds"
        @item:dblclick="handleItemSelect"
        @item:keydown="handleItemSelect"
      >
        <template #item="slotProps">
          {{slotProps.name}}
        </template>
      </NativeListbox>
    </div>
    <div class="controls">
      <div>
        <button @click="openEntryKindListModal">Изменить</button>
      </div>
      <div>
        <button @click="handleSubmit" :disabled="selectedEntryKindIdx === null">
          Выбрать
        </button>
        <button @click="handleCancel">Отменить</button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/mixins";

.content {
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;
  @include user-select-none;

  input {
    width: 100%;
    padding: 0.3em 0.6em;
  }
}

.wrapper {
  flex: 1;
}

.list {
  height: 100%;
  padding: 4px 0;
  border-radius: 4px;
  box-sizing: border-box;
  background-color: #272727;
}

.controls {
  display: flex;
  justify-content: space-between;
  gap: 5px;

  div {
    display: flex;
    gap: 5px;
  }
}
</style>
