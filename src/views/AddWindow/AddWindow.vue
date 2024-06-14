<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, TauriEvent, emit } from '@tauri-apps/api/event';

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
import Banner from 'components/Banner.vue';

const entryKinds = computed(() => entryKindStore.cached_all);
const decks = computed(() => deckStore.cached_all);

const editorRef = ref<EditorExposed | null>(null);
const selectedEntryKind = ref<EntryKind | null>(null);
const selectedDeck = ref<Deck | null>(null);

const createdEntry = ref(false);
const disabled = ref(false);
const loading = ref(false);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(dataEvents.update.entryKind, load);
useTauriEvent(uiEvents.EntryKindFilterModal.onResult, handleEntryKindSelected);
useTauriEvent(uiEvents.DeckFilterModal.onResult, handleDeckSelected);
useTauriEvent(uiEvents.window_open, load);

onMounted(() => {
  load();
});

function reset() {
  editorRef.value?.clear();

  createdEntry.value = false;
  disabled.value = false;
  loading.value = false;
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
  const tags = editorRef.value.getTags();
  if (values.length === 0)
    return;

  const allBlank = !values.some((el) => el !== "");
  if (allBlank) {
    if (!disabled.value) {
      disabled.value = true;
      setTimeout(() => {
        disabled.value = false;
      }, 2000);
    }

    return;
  }

  loading.value = true;
  entryStore.create(
    selectedEntryKind.value.id,
    selectedDeck.value.id,
    values,
    tags
  )
    .then(() => {
      emit(dataEvents.update.entry);
      loading.value = false;
      createdEntry.value = true;
      if (editorRef.value)
        editorRef.value.clear();
    });
}

function handleClose() {
  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}
</script>

<template>
  <div class="content">
    <Banner v-model="createdEntry">Запись успешно добавлена</Banner>
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
      <span class="controls__message" v-show="disabled">
        *Заполните хотя бы одно поле
      </span>
      <button
        :class="{ shake: disabled }"
        :disabled="loading"
        @click="handleConfirm"
      >
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

.shake {
  animation: shake 0.82s cubic-bezier(0.36, 0.07, 0.19, 0.97) both;
  transform: translate3d(0, 0, 0);
}

@keyframes shake {
  10%,
  90% {
    transform: translate3d(-1px, 0, 0);
  }

  20%,
  80% {
    transform: translate3d(2px, 0, 0);
  }

  30%,
  50%,
  70% {
    transform: translate3d(-4px, 0, 0);
  }

  40%,
  60% {
    transform: translate3d(4px, 0, 0);
  }
}

.content {
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
  gap: 8px;
  @include user-select-none;
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
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
}

.controls__message {
  color: red;
  margin-right: 2px;
}
</style>
