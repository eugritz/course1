<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, emit } from '@tauri-apps/api/event';

import { Deck } from 'entities/Deck';
import { EntryKind } from 'entities/EntryKind';

import { entryKindStore } from 'stores/entryKindStore';
import { deckStore } from 'stores/deckStore';
import { useTauriEvent } from 'utils/tauriEvent';
import events from 'constants/events';

const entryKinds = computed(() => entryKindStore.cached_all);
const decks = computed(() => deckStore.cached_all);
const selectedEntryKind = ref<EntryKind | null>(null);
const selectedDeck = ref<Deck | null>(null);

useTauriEvent(events.EntryKindAddModal.onResult, load);
useTauriEvent(events.EntryKindFilterModal.onResult, handleEntryKindSelected);
useTauriEvent(events.DeckFilterModal.onResult, handleDeckSelected);
useTauriEvent(events.window_open, load);

onMounted(() => {
  load();
});

function load() {
  entryKindStore.all();
  entryKindStore.last().then((entryKind) => {
    selectedEntryKind.value = entryKind;
  });

  deckStore.all();
  deckStore.last().then((deck) => {
    selectedDeck.value = deck;
  });
}

function openEntryKindFilterModal() {
  emit(events.EntryKindFilterModal.setData, {
    selectedEntryKindId: selectedEntryKind.value?.id,
  }).then(() => {
    invoke(events.EntryKindFilterModal.open);
  });
}

function openDeckFilterModal() {
  emit(events.DeckFilterModal.setData, {
    selectedDeckId: selectedDeck.value?.id,
  }).then(() => {
    invoke(events.DeckFilterModal.open);
  });
}

function handleEntryKindSelected(event: Event<unknown>) {
  const payload = event.payload as {
    selectedEntryKindId: number,
  };

  selectedEntryKind.value = entryKinds.value.find((entryKind) =>
    entryKind.id === payload.selectedEntryKindId) ?? null;
}

function handleDeckSelected(event: Event<unknown>) {
  const payload = event.payload as {
    selectedDeckId: number,
  };

  selectedDeck.value = decks.value.find((deck) =>
    deck.id === payload.selectedDeckId) ?? null;
}
</script>

<template>
  <div class="content">
    <div class="record">
      <div class="record__kind">
        Вид
        <button @click="openEntryKindFilterModal">
          {{selectedEntryKind?.name ?? "Вид записи"}}
        </button>
      </div>
      <div class="record__deck">
        Колода
        <button @click="openDeckFilterModal">
          {{selectedDeck?.name ?? "Колода"}}
        </button>
      </div>
    </div>
    <div class="wrapper">
    </div>
    <div class="controls">
      <button>Добавить</button>
      <button>Закрыть</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/mixins";

.content {
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
}

.record {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;
  @include user-select-none;
}

.record__kind {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;

  button {
    width: 100%;
    padding: 0.3em 0.6em;
  }
}

.record__deck {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;

  button {
    width: 100%;
    padding: 0.3em 0.6em;
  }
}

.wrapper {
  flex: 1;
}

.controls {
  display: flex;
  justify-content: flex-end;
  gap: 5px;
}
</style>
