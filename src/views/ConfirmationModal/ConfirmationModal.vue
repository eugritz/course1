<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { Event, TauriEvent } from "@tauri-apps/api/event";

import { useTauriEvent } from "utils/tauriEvent";
import uiEvents from "constants/uiEvents";

import Loader from "components/Loader.vue";

const id = ref<string | null>(null);
const message = ref("");
const loading_ = ref(false);
const loading = ref<number | null>(null);
const payload = ref<unknown>(null);
const parent = ref<string>("");

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(uiEvents.ConfirmationModal.setData, handleSetData);
useTauriEvent(uiEvents.ConfirmationModal.onReady, handleReady);

function reset(event?: Event<unknown>) {
  if (event && event.windowLabel !== "ConfirmationModal")
    return;

  message.value = "";
  loading_.value = false;
  loading.value = null;
}

function handleSetData(event: Event<unknown>) {
  const payload_ = event.payload as {
    id: string | null,
    title: string,
    message: string,
    loading: boolean | null,
    payload: unknown,
    parent: string,
  };

  id.value = payload_.id;
  message.value = payload_.message;
  loading_.value = payload_.loading ?? false;
  payload.value = payload_.payload ?? null;
  parent.value = payload_.parent;
}

function handleConfirm() {
  invoke("confirmation_modal_on_result", {
    id: id.value,
    button: 1,
    payload: payload.value,
    parent: parent.value,
  });

  if (loading_.value) {
    loading.value = 1;
    return;
  }

  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}

function handleCancel() {
  invoke("confirmation_modal_on_result", {
    id: id.value,
    button: 0,
    payload: payload.value,
    parent: parent.value,
  });

  if (loading_.value) {
    loading.value = 0;
    return;
  }

  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}

function handleReady() {
  if (loading.value === null) {
    return;
  }

  invoke(uiEvents.window_close).then(() => {
    reset();
  });
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
      <button type="button" :disabled="loading === 0" @click="handleCancel">
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
.hidden {
  opacity: 0;
}

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
</style>
