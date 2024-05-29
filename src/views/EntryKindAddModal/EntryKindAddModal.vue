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
useTauriEvent(uiEvents.InputModal.onResult, handleEntryKindNameResult);
useTauriEvent(uiEvents.window_open, load);

onMounted(() => {
  load();
  reset();
});

function load() {
  entryKindStore.all();
}

function reset(event?: Event<unknown>) {
  if (event && event.windowLabel !== "EntryKindAddModal")
    return;

  if (listRef.value) {
    listRef.value.focus();
    listRef.value.select(0);
  }
}

function handleEntryKindNameResult(event: Event<unknown>) {
  if (!selectedEntryKind.value)
    return;

  const payload = event.payload as {
    input: string,
  };

  entryKindStore.create(selectedEntryKind.value.id, payload.input).finally(() => {
    emit(dataEvents.update.entryKind).then(() => {
      emit(uiEvents.InputModal.onReady);
      invoke(uiEvents.window_close).then(() => {
        reset();
      });
    });
  });
}

function handleItemSelect(item: EntryKind) {
  invoke(uiEvents.InputModal.open, {
    title: "Добавить вид записи",
    label: "Имя нового вида записи",
    value: item.name,
    placeholder: item.name,
    buttonText: "Добавить",
    loading: true,
  });
}

function handleSubmit() {
  if (!selectedEntryKind.value)
    return;

  handleItemSelect(selectedEntryKind.value);
}

function handleCancel() {
  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}
</script>

<template>
  <div class="content" @keydown.esc="handleCancel">
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
          {{
            slotProps.default
            ? "Добавить: " + slotProps.name
            : "Клонировать: " + slotProps.name
          }}
        </template>
      </NativeListbox>
    </div>
    <div class="controls">
      <button @click="handleSubmit" :disabled="selectedEntryKindIdx === null">
        Выбрать
      </button>
      <button @click="handleCancel">Отменить</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/mixins";

button {
  padding: 0.3em 0.6em;
}

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
  justify-content: flex-end;
  gap: 5px;
}
</style>
