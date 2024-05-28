<script setup lang="ts">
import { onMounted, ref } from "vue";
import { emit, Event, TauriEvent } from "@tauri-apps/api/event";

import { Deck } from "entities/Deck";
import { deckStore } from "stores/deckStore";
import { useTauriEvent } from "utils/tauriEvent";
import events from "constants/events";

import Loader from "components/Loader.vue";
import {invoke} from "@tauri-apps/api";

const loading = ref(false);
const deck = ref<Deck | null>(null);
const deckTitle = ref("");
const deckTitleRef = ref<HTMLInputElement | null>(null);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(events.DeckRenameModal.setData, handleSetData);

onMounted(() => {
  deckTitleRef.value?.focus();
});

function reset(event?: Event<unknown>) {
  if (event && event.windowLabel !== "DeckRenameModal")
    return;

  loading.value = false;
  deckTitle.value = "";
  deckTitleRef.value?.focus();
}

function renameDeck() {
  if (!deck.value) {
    return;
  }

  loading.value = true;
  deckStore.rename(deck.value.id, deckTitle.value).finally(() => {
    emit(events.DeckRenameModal.onResult);
    invoke(events.window_close).then(() => {
      reset();
    });
  });
}

function handleSetData(event: Event<unknown>) {
  console.log("set data");
  const payload = event.payload as {
    deck?: Deck,
  };

  if (payload.deck !== undefined) {
    deck.value = payload.deck;
    deckTitle.value = payload.deck.name;
  }
}

function handleCancel() {
  invoke(events.window_close).then(() => {
    reset();
  });
}
</script>

<template>
  <form
    class="dialog"
    @submit.prevent="renameDeck"
    @keydown.esc="handleCancel"
  >
    <label>Новое название колоды</label>
    <input
      class="deck-title"
      type="text"
      :placeholder="deck?.name"
      v-model="deckTitle"
      ref="deckTitleRef"
    />
    <div class="dialog__controls">
      <button type="button" @click="handleCancel">Отменить</button>
      <button type="submit" :disabled="loading">
        <Loader v-show="loading" />
        <span :class="{ hidden: loading }">Изменить</span>
      </button>
    </div>
  </form>
</template>

<style scoped lang="scss">
.dialog {
  width: 100%;

  input,
  button {
    margin: 5px 0px;
  }
}

.dialog__controls {
  display: flex;
  justify-content: flex-end;
  gap: 5px
}

.deck-title {
  width: 100%;
}

.hidden {
  opacity: 0;
}
</style>
