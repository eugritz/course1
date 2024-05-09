<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { Event, TauriEvent } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

import { useTauriEvent } from "utils/tauriEvent";
import events from "constants/events";

import Loader from "components/Loader.vue";

const message = ref("");
const loading_ = ref(false);
const loading = ref<number | null>(null);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(events.ConfirmationModal.setData, handleSetData);
useTauriEvent(events.ConfirmationModal.onReady, handleReady);

function reset() {
  message.value = "";
  loading_.value = false;
  loading.value = null;
}

function handleSetData(event: Event<unknown>) {
  const payload = event.payload as {
    title: string,
    message: string,
    loading: boolean,
  };

  message.value = payload.message;
  loading_.value = payload.loading;
}

function handleConfirm() {
  invoke("confirmation_modal_on_result", {
    button: 1,
  });

  if (loading_.value) {
    loading.value = 1;
    return;
  }

  reset();
  appWindow.hide();
}

function handleCancel() {
  invoke("confirmation_modal_on_result", {
    button: 0,
  });

  if (loading_.value) {
    loading.value = 0;
    return;
  }

  reset();
  appWindow.hide();
}

function handleReady() {
  if (loading.value === null) {
    return;
  }

  reset();
  appWindow.hide();
}
</script>

<template>
  <form
    class="dialog"
    @submit.prevent="handleConfirm"
    @keydown.esc="handleCancel"
  >
    <p>{{ message }}</p>
    <div class="dialog__controls">
      <button :disabled="loading === 0" @click="handleCancel">
        <Loader v-show="loading === 0" />
        <span :class="{ hidden: loading === 0 }">Нет</span>
      </button>
      <button type="submit" :disabled="loading === 1">
        <Loader v-show="loading === 1" />
        <span :class="{ hidden: loading === 1 }">Да</span>
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

.hidden {
  opacity: 0;
}
</style>
