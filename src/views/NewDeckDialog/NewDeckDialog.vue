<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { listen, TauriEvent, UnlistenFn } from "@tauri-apps/api/event";

const deckTitle = ref("");
const deckTitleRef = ref<HTMLInputElement | null>(null);

function reset() {
  deckTitleRef.value?.focus();
  deckTitle.value = "";
}

let unlistenOnWindowClosed: UnlistenFn | undefined;
onMounted(async () => {
  unlistenOnWindowClosed = await listen(
    TauriEvent.WINDOW_CLOSE_REQUESTED,
    (_) => {
      reset();
    }
  );
});

onUnmounted(() => {
  if (unlistenOnWindowClosed) {
    unlistenOnWindowClosed();
  }
});

function createNewDeck() {
}

function handleCancel() {
  reset();
  appWindow.hide();
}
</script>

<template>
  <form class="dialog" @submit.prevent="createNewDeck">
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
