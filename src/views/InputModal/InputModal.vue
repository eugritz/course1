<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { Event, TauriEvent } from "@tauri-apps/api/event";

import { useTauriEvent } from "utils/tauriEvent";
import events from "constants/events";

import Loader from "components/Loader.vue";

const label = ref("");
const input = ref("");
const inputRef = ref<HTMLInputElement | null>(null);
const placeholder = ref("");
const buttonText = ref("OK");
const loading_ = ref(false);
const loading = ref(false);
const parent = ref("");

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(events.InputModal.setData, handleSetData);
useTauriEvent(events.InputModal.onReady, handleReady);

onMounted(() => {
  inputRef.value?.focus();
});

function reset(event?: Event<unknown>) {
  if (event && event.windowLabel !== "InputModal")
    return;

  input.value = "";
  inputRef.value?.focus();
  loading_.value = false;
  loading.value = false;
}

function handleSetData(event: Event<unknown>) {
  const payload = event.payload as {
    title: string,
    label: string,
    value: string | null,
    placeholder: string | null,
    buttonText: string | null,
    loading: boolean | null,
    parent: string,
  };

  label.value = payload.label;
  input.value = payload.value ?? "";
  placeholder.value = payload.placeholder ?? "";
  buttonText.value = payload.buttonText ?? "OK";
  loading_.value = payload.loading ?? false;
  parent.value = payload.parent;
}

function handleConfirm() {
  invoke("input_modal_on_result", {
    input: input.value,
    parent: parent.value,
  });

  if (loading_.value) {
    loading.value = true;
    return;
  }

  invoke(events.window_close).then(() => {
    reset();
  });
}

function handleCancel() {
  invoke(events.window_close).then(() => {
    reset();
  });
}

function handleReady() {
  if (!loading.value) {
    return;
  }

  invoke(events.window_close).then(() => {
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
    <label>{{label}}</label>
    <input
      class="dialog__input"
      type="text"
      :placeholder="placeholder"
      v-model="input"
      ref="inputRef"
    />
    <div class="dialog__controls">
      <button type="button" @click="handleCancel">Отменить</button>
      <button type="submit" :disabled="loading">
        <Loader v-show="loading" />
        <span :class="{ hidden: loading }">{{buttonText}}</span>
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

.dialog__input {
  width: 100%;
}

.hidden {
  opacity: 0;
}
</style>