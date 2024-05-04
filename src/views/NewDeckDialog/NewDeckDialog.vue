<script setup lang="ts">
import { onMounted,
ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { emit, TauriEvent } from "@tauri-apps/api/event";
import { useTauriEvent } from "utils/tauriEvent";

const deckTitle = ref("");
const deckTitleRef = ref<HTMLInputElement | null>(null);

function reset() {
  deckTitleRef.value?.focus();
  deckTitle.value = "";
}

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);

onMounted(() => {
  deckTitleRef.value?.focus();
});

function createNewDeck() {
  emit("dialog_result", {
    deckTitle: deckTitle.value,
  });
  reset();
  appWindow.hide();
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
      <button type="submit">Создать</button>
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
</style>
