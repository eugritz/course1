<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { Event, TauriEvent } from "@tauri-apps/api/event";

import { useTauriEvent } from "utils/tauriEvent";
import uiEvents from "constants/uiEvents";

import Loader from "components/Loader.vue";

const id = ref<string | null>(null);
const label = ref("");
const input = ref("");
const inputRef = ref<HTMLInputElement | null>(null);
const placeholder = ref("");
const buttonText = ref("OK");
const loading_ = ref(false);
const loading = ref(false);
const payload = ref<unknown>(null);
const parent = ref("");

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(uiEvents.InputModal.setData, handleSetData);
useTauriEvent(uiEvents.InputModal.onReady, handleReady);

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
  const payload_ = event.payload as {
    id: string | null,
    title: string,
    label: string,
    value: string | null,
    placeholder: string | null,
    buttonText: string | null,
    loading: boolean | null,
    payload: unknown,
    parent: string,
  };

  id.value = payload_.id,
  label.value = payload_.label;
  input.value = payload_.value ?? "";
  placeholder.value = payload_.placeholder ?? "";
  buttonText.value = payload_.buttonText ?? "OK";
  loading_.value = payload_.loading ?? false;
  payload.value = payload_.value ?? null;
  parent.value = payload_.parent;
}

function handleConfirm() {
  invoke("input_modal_on_result", {
    id: id.value,
    input: input.value,
    payload: payload.value,
    parent: parent.value,
  });

  if (loading_.value) {
    loading.value = true;
    return;
  }

  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}

function handleCancel() {
  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}

function handleReady() {
  if (!loading.value) {
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
