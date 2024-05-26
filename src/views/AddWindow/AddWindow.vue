<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { Event, emit } from '@tauri-apps/api/event';

import { Deck } from 'entities/Deck';
import { deckStore } from 'stores/deckStore';
import { useTauriEvent } from 'utils/tauriEvent';
import events from 'constants/events';

const decks = computed(() => deckStore.cached_all);
const selectedDeck = ref<Deck | null>(null);

useTauriEvent(events.DeckFilterModal.onResult, handleDeckSelected);
useTauriEvent(events.window_open, load);

onMounted(() => {
  load();
});

function load() {
  deckStore.all();
  deckStore.last().then((deck) => {
    selectedDeck.value = deck;
  });
}

function openDeckFilterModal() {
  emit(events.DeckFilterModal.setData, {
    selectedDeckId: selectedDeck.value?.id,
  }).then(() => {
    invoke(events.DeckFilterModal.open);
  });
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
        <button>Вид</button>
      </div>
      <div class="record__deck">
        Колода
        <button @click="openDeckFilterModal">{{selectedDeck?.name}}</button>
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
