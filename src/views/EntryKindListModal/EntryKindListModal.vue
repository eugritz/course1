<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, TauriEvent, emit } from '@tauri-apps/api/event';

import { EntryKind } from 'entities/EntryKind';
import { entryKindStore } from 'stores/entryKindStore';
import { useTauriEvent } from 'utils/tauriEvent';
import uiEvents from 'constants/uiEvents';

import NativeListbox, { NativeListboxExposed } from 'components/NativeListbox.vue';

const listRef = ref<NativeListboxExposed | null>(null);

const selectedEntryKind = ref<EntryKind | null>(null);
const selectedEntryKindIdx = ref<number | null>(null);

const entryKinds = computed(() => entryKindStore.cached_all);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(uiEvents.EntryKindAddModal.onResult, load);
useTauriEvent(uiEvents.EntryKindListModal.setData, handleSetData);
useTauriEvent(uiEvents.InputModal.onResult, handleRenameEntryKindResult);
useTauriEvent(uiEvents.window_open, load);

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

  if (listRef.value)
    listRef.value.deselect();
}

function handleSetData(event: Event<unknown>) {
  const payload = event.payload as {
    selectedEntryKindId: number,
  };

  let found = entryKinds.value.findIndex((deck) =>
    deck.id == payload.selectedEntryKindId);
  selectedEntryKindIdx.value = found < 0 ? null : found;
}

function handleAddEntryKind() {
  invoke(uiEvents.EntryKindAddModal.open);
}

function handleRenameEntryKind() {
  if (!selectedEntryKind.value)
    return;

  invoke(uiEvents.InputModal.open, {
    title: "Переименовать вид записи",
    label: "Новое имя вида записи",
    value: selectedEntryKind.value.name,
    placeholder: selectedEntryKind.value.name,
    buttonText: "Изменить",
    loading: true,
  });
}

function handleRenameEntryKindResult(event: Event<unknown>) {
  if (!selectedEntryKind.value)
    return;

  const payload = event.payload as {
    input: string
  };

  console.log("rename", payload.input);
  entryKindStore
    .rename(selectedEntryKind.value.id, payload.input)
    .finally(
      () => {
        emit(uiEvents.InputModal.onReady);
        load();
      }
    );
}

function handleDeleteEntryKind() {
  // TODO
}

function handleClose() {
  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}
</script>

<template>
  <div class="content" @keydown.esc="handleClose">
    <div class="wrapper">
      <NativeListbox
        v-model="selectedEntryKind"
        v-model:index="selectedEntryKindIdx"
        ref="listRef"
        class="list"
        :items="entryKinds"
      >
        <template #item="slotProps">
          {{slotProps.name}}
        </template>
      </NativeListbox>
    </div>
    <div class="controls">
      <button @click="handleAddEntryKind">Добавить</button>
      <button
        @click="handleRenameEntryKind"
        :disabled="selectedEntryKind?.default"
      >
        Переименовать
      </button>
      <button
        @click="handleDeleteEntryKind"
        :disabled="selectedEntryKind?.default"
      >
        Удалить
      </button>
      <button @click="handleClose">Закрыть</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/mixins";

button {
  padding: 0.3em 1.2em;
}

.content {
  height: calc(100vh - 16px);
  display: flex;
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
  flex-direction: column;
  gap: 5px;
}
</style>
