<script setup lang="ts">
import { onMounted, ref } from "vue";
import { emit, TauriEvent } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

import { deckStore } from "stores/deckStore";
import { useTauriEvent } from "utils/tauriEvent";

import Loader from "components/Loader.vue";
import events from "constants/events";

const loading = ref(false);
const deckTitle = ref("");
const deckTitleRef = ref<HTMLInputElement | null>(null);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);

onMounted(() => {
  deckTitleRef.value?.focus();
});

function reset() {
  loading.value = false;
  deckTitleRef.value?.focus();
  deckTitle.value = "";
}

function createNewDeck() {
  loading.value = true;
  deckStore.create(deckTitle.value).finally(() => {
    emit(events.NewDeckModal.onResult, {
      deckTitle: deckTitle.value,
    });
    reset();
    appWindow.hide();
  });
}

function handleCancel() {
  reset();
  appWindow.hide();
}
</script>

<template>
  <form
    class="dialog"
    @submit.prevent="createNewDeck"
    @keydown.esc="handleCancel"
  >
    <label>Название колоды</label>
    <input
      class="deck-title"
      type="text"
      v-model="deckTitle"
      ref="deckTitleRef"
    />
    <div class="dialog__controls">
      <button type="button" @click="handleCancel">Отменить</button>
      <button type="submit" :disabled="loading">
        <Loader v-show="loading" />
        <span :class="{ hidden: loading }">Создать</span>
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
