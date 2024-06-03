<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, emit } from '@tauri-apps/api/event';

import { Deck } from 'entities/Deck';
import { EntryKind } from 'entities/EntryKind';

import { deckStore } from 'stores/deckStore';
import { entryKindStore } from 'stores/entryKindStore';
import { entryStore } from 'stores/entryStore';

import { useTauriEvent } from 'utils/tauriEvent';
import dataEvents from 'constants/dataEvents';
import uiEvents from 'constants/uiEvents';

import Editor, { EditorExposed } from 'components/Editor.vue';
import Loader from 'components/Loader.vue';

const editorRef = ref<EditorExposed | null>(null);
const entryKinds = computed(() => entryKindStore.cached_all);
const decks = computed(() => deckStore.cached_all);
const selectedEntryKind = ref<EntryKind | null>(null);
const selectedDeck = ref<Deck | null>(null);
const loading = ref(false);

useTauriEvent(dataEvents.update.entryKind, load);
useTauriEvent(uiEvents.EntryKindFilterModal.onResult, handleEntryKindSelected);
useTauriEvent(uiEvents.DeckFilterModal.onResult, handleDeckSelected);
useTauriEvent(uiEvents.window_open, load);

onMounted(() => {
  load();
});

function reset() {
  if (editorRef.value)
    editorRef.value.clear();
}

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
  emit(uiEvents.EntryKindFilterModal.setData, {
    selectedEntryKindId: selectedEntryKind.value?.id,
  }).then(() => {
    invoke(uiEvents.EntryKindFilterModal.open);
  });
}

function openDeckFilterModal() {
  emit(uiEvents.DeckFilterModal.setData, {
    selectedDeckId: selectedDeck.value?.id,
  }).then(() => {
    invoke(uiEvents.DeckFilterModal.open);
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

function handleConfirm() {
  if (!editorRef.value || !selectedEntryKind.value || !selectedDeck.value)
    return;

  const values = editorRef.value.getValues();
  if (values.length === 0)
    return;

  loading.value = true;
  entryStore.create(selectedEntryKind.value.id, selectedDeck.value.id, values)
    .then(() => {
      loading.value = false;
      reset();
    });
}

function handleClose() {
  invoke(uiEvents.window_close);
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
      <Editor ref="editorRef" :entry-kind-id="selectedEntryKind?.id" />
    </div>
    <div class="controls">
      <button type="button" :disabled="loading" @click="handleConfirm">
        <Loader v-show="loading" />
        <span :class="{ hidden: loading }">Добавить</span>
      </button>
      <button @click="handleClose">Закрыть</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/mixins";

.hidden {
  opacity: 0;
}

.content {
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
  gap: 8px
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
