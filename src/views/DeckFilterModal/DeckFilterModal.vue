<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, TauriEvent, emit } from '@tauri-apps/api/event';

import { Deck } from 'entities/Deck';
import { deckStore } from 'stores/deckStore';
import { useTauriEvent } from 'utils/tauriEvent';
import uiEvents from 'constants/uiEvents';

import NativeListbox, { NativeListboxExposed } from 'components/NativeListbox.vue';

const filter = ref("");
const filterRef = ref<HTMLElement | null>(null);
const listRef = ref<NativeListboxExposed | null>(null);

const selectedDeck = ref<Deck | null>(null);
const selectedDeckIdx = ref<number | null>(null);

const decks = computed(() =>
  deckStore.cached_all.filter(
    (deck) =>
      deck.name.toLowerCase().includes(filter.value.trim().toLowerCase())));

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(uiEvents.DeckFilterModal.setData, handleSetData);
useTauriEvent(uiEvents.window_open, load);

onMounted(() => {
  load();
  reset();
});

function load() {
  deckStore.all();
}

function reset(event?: Event<unknown>) {
  if (event && event.windowLabel !== "DeckFilterModal")
    return;

  filter.value = "";
  if (filterRef.value)
    filterRef.value.focus();
  if (listRef.value)
    listRef.value.deselect();
}

function handleSetData(event: Event<unknown>) {
  const payload = event.payload as {
    selectedDeckId: number,
  };

  let found = decks.value.findIndex((deck) => deck.id == payload.selectedDeckId);
  selectedDeckIdx.value = found < 0 ? null : found;
}

function handleItemSelect(item: Deck) {
  emit(uiEvents.DeckFilterModal.onResult, {
    selectedDeckId: item.id,
  }).then(() => {
    invoke(uiEvents.window_close).then(() => {
      reset();
    });
  });
}

function handleSubmit() {
  if (!selectedDeck.value)
    return;

  handleItemSelect(selectedDeck.value);
}

function handleCancel() {
  invoke(uiEvents.window_close).then(() => {
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
        v-model="selectedDeck"
        v-model:index="selectedDeckIdx"
        ref="listRef"
        class="list"
        :items="decks"
        @item:dblclick="handleItemSelect"
        @item:keydown="handleItemSelect"
      >
        <template #item="slotProps">
          {{slotProps.name}}
        </template>
      </NativeListbox>
    </div>
    <div class="controls">
      <button @click="handleSubmit" :disabled="selectedDeckIdx === null">
        Выбрать
      </button>
      <button @click="handleCancel">Отменить</button>
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
  justify-content: flex-end;
  gap: 5px;
}
</style>
