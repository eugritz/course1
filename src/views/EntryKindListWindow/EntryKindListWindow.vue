<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, TauriEvent, emit } from '@tauri-apps/api/event';

import { EntryKind } from 'entities/EntryKind';
import { entryKindStore } from 'stores/entryKindStore';
import { useTauriEvent } from 'utils/tauriEvent';
import dataEvents from 'constants/dataEvents';
import uiEvents from 'constants/uiEvents';

import NativeListbox, { NativeListboxExposed } from 'components/NativeListbox.vue';

const listRef = ref<NativeListboxExposed | null>(null);

const selectedEntryKind = ref<EntryKind | null>(null);
const selectedEntryKindIdx = ref<number | null>(null);

const entryKinds = computed(() => entryKindStore.cached_all);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(dataEvents.update.entryKind, load);
useTauriEvent(uiEvents.EntryKindListWindow.setData, handleSetData);
useTauriEvent(uiEvents.InputModal.onResult, handleRenameEntryKindResult);
useTauriEvent(uiEvents.ConfirmationModal.onResult, handleDeleteEntryKindResult);
useTauriEvent(uiEvents.window_open, load);

onMounted(() => {
  load();
  reset();
});

function load() {
  entryKindStore.all();
}

function reset(event?: Event<unknown>) {
  if (event && event.windowLabel !== "EntryKindListWindow")
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
  const payload = event.payload as {
    input: string,
  };

  const newName = payload.input.trim();

  if (!selectedEntryKind.value
      || newName === selectedEntryKind.value.name
      || newName.length == 0) {
    emit(uiEvents.InputModal.onReady);
    return;
  }

  entryKindStore
    .rename(selectedEntryKind.value.id, payload.input)
    .then(
      () => {
        emit(dataEvents.update.entryKind);
      }
    )
    .finally(
      () => {
        emit(uiEvents.InputModal.onReady);
      }
    );
}

function handleDeleteEntryKind() {
  invoke(uiEvents.ConfirmationModal.open, {
    title: "Удалить вид записи",
    message: "Вы уверены, что хотите удалить вид записи?",
    loading: true,
  });
}

function handleDeleteEntryKindResult(event: Event<unknown>) {
  const payload = event.payload as {
    button: number,
  };

  if (!selectedEntryKind.value || payload.button !== 1) {
    emit(uiEvents.ConfirmationModal.onReady);
    return;
  }

  entryKindStore.delete(selectedEntryKind.value.id).then(() => {
    emit(dataEvents.update.entryKind);
  }).finally(() => {
    emit(uiEvents.ConfirmationModal.onReady);
  });
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
        :disabled="selectedEntryKind?.immutable"
      >
        Переименовать
      </button>
      <button
        @click="handleDeleteEntryKind"
        :disabled="selectedEntryKind?.immutable"
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
