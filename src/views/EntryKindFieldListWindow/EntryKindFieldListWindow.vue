<script setup lang="ts">
import { nextTick, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { TauriEvent, Event as UiEvent, emit } from '@tauri-apps/api/event';

import { EntryKindField } from 'entities/EntryKindField';
import { entryKindFieldStore } from 'stores/entryKindFieldStore';
import { useTauriEvent } from 'utils/tauriEvent';
import dataEvents from 'constants/dataEvents';
import uiEvents from 'constants/uiEvents';

import Loader from 'components/Loader.vue';
import NativeListbox, { NativeListboxExposed } from 'components/NativeListbox.vue';

const listRef = ref<NativeListboxExposed | null>(null);

const entryKindId = ref<number | null>(null);
const selectedField = ref<EntryKindField | null>(null);

const fields = ref<EntryKindField[]>([]);
const deletedFields = ref<number[]>([]);
const defaultField = ref<EntryKindField | null>(null);
const fieldDescription = ref("");

const disabled = ref(false);
const loading = ref(false);

useTauriEvent(TauriEvent.WINDOW_CLOSE_REQUESTED, reset);
useTauriEvent(uiEvents.EntryKindFieldListWindow.setData, handleSetData);
useTauriEvent(uiEvents.InputModal.onResult, handleInputDialogResult);
useTauriEvent(uiEvents.ConfirmationModal.onResult, handleDeleteFieldResult);

function reset() {
  deletedFields.value = [];
  fieldDescription.value = "";
  loading.value = false;
}

function load() {
  if (entryKindId.value === null)
    return;

  entryKindFieldStore.get_fields(entryKindId.value).then((fields_) => {
    fields.value = fields_;
    defaultField.value = fields_.find((field) => field.is_default) ?? null;

    nextTick(() => {
      if (listRef.value) {
        listRef.value.focus();
        listRef.value.select(0);
      }
    });
  });
}

function handleSetData(event: UiEvent<unknown>) {
  const payload = event.payload as {
    entryKindId: number,
  };

  entryKindId.value = payload.entryKindId;
  load();
}

function handleAddField() {
  invoke(uiEvents.InputModal.open, {
    id: "add",
    title: "Добавить поле",
    label: "Имя нового поля",
    buttonText: "Добавить",
  });
}

function handleRenameField() {
  if (!selectedField.value)
    return;

  invoke(uiEvents.InputModal.open, {
    id: "rename",
    title: "Переименовать поле",
    label: "Новое имя поля",
    value: selectedField.value.name,
    placeholder: selectedField.value.name,
    buttonText: "Изменить",
  });
}

function handleInputDialogResult(event: UiEvent<unknown>) {
  const payload = event.payload as {
    id: string,
    input: string,
  };

  if (payload.id === "add")
    handleAddFieldResult(event);
  else if (payload.id === "rename")
    handleRenameFieldResult(event);
}

function handleAddFieldResult(event: UiEvent<unknown>) {
  const payload = event.payload as {
    input: string,
  };

  const newName = payload.input.trim();
  if (newName.length == 0) {
    return;
  }

  if (!selectedField.value) {
    fields.value.push(<EntryKindField>{
      id: -1,
      entry_kind_id: -1,
      order: 1,
      name: newName,
      desc: "",
      type: "ANY",
      immutable: false,
      is_default: true,
    });
    defaultField.value = fields.value[0];
    listRef.value?.selectNext();
    return;
  }

  for (let i = selectedField.value.order; i < fields.value.length; i++) {
    fields.value[i].order += 1;
  }

  fields.value.splice(selectedField.value.order, 0, <EntryKindField>{
    id: -1,
    entry_kind_id: -1,
    order: selectedField.value.order + 1,
    name: newName,
    desc: "",
    type: "ANY",
    immutable: false,
    is_default: false,
  });

  listRef.value?.selectNext();
}

function handleRenameFieldResult(event: UiEvent<unknown>) {
  const payload = event.payload as {
    input: string,
  };

  const newName = payload.input.trim();

  if (!selectedField.value
      || newName === selectedField.value.name
      || newName.length == 0) {
    return;
  }

  selectedField.value.name = newName;
}

function handleDeleteField() {
  invoke(uiEvents.ConfirmationModal.open, {
    title: "Удалить поле",
    message: "Вы уверены, что хотите удалить данное поле?",
    loading: false,
  });
}

function handleDeleteFieldResult(event: UiEvent<unknown>) {
  const payload = event.payload as {
    button: number,
  };

  if (!selectedField.value || payload.button !== 1) {
    return;
  }

  if (selectedField.value.is_default && fields.value.length > 1) {
    if (selectedField.value.order > 1) {
      fields.value[selectedField.value.order - 2].is_default = true;
      defaultField.value = fields.value[selectedField.value.order - 2];
    } else {
      fields.value[selectedField.value.order].is_default = true;
      defaultField.value = fields.value[selectedField.value.order];
    }
  }

  for (let i = selectedField.value.order; i < fields.value.length; i++) {
    fields.value[i].order -= 1;
  }

  if (selectedField.value.id > 0) {
    deletedFields.value.push(selectedField.value.id);
  }

  deletedFields.value.push();
  fields.value.splice(selectedField.value.order - 1, 1);
  listRef.value?.selectPrev();
}

function handleMoveUpField() {
  if (!selectedField.value)
    return;

  fields.value[selectedField.value.order - 2].order += 1;
  selectedField.value.order -= 1;
  fields.value = fields.value.sort((a, b) => a.order - b.order);
  listRef.value?.selectPrev();
}

function handleMoveDownField() {
  if (!selectedField.value)
    return;

  fields.value[selectedField.value.order].order -= 1;
  selectedField.value.order += 1;
  fields.value = fields.value.sort((a, b) => a.order - b.order);
  listRef.value?.selectNext();
}

function handleFieldSelected(field: EntryKindField) {
  fieldDescription.value = field.desc;
}

function handleFieldDescriptionChanged() {
  if (!selectedField.value)
    return;

  selectedField.value.desc = fieldDescription.value;
}

function handleDefaultFieldChanged() {
  if (!defaultField.value || !selectedField.value)
    return;

  defaultField.value.is_default = false;
  selectedField.value.is_default = true;
  defaultField.value = selectedField.value;
}

function handleConfirm() {
  if (entryKindId.value === null)
    return;

  if (fields.value.length === 0) {
    if (!disabled.value) {
      disabled.value = true;
      setTimeout(() => {
        disabled.value = false;
      }, 2000);
    }
    return;
  }

  loading.value = true;
  entryKindFieldStore
    .update_fields(entryKindId.value, fields.value, deletedFields.value)
    .then(() => {
      emit(dataEvents.update.entryKindField);
    }).finally(() => {
      invoke(uiEvents.window_close).then(() => {
        reset();
      });
    });
}

function handleCancel() {
  invoke(uiEvents.window_close).then(() => {
    reset();
  });
}
</script>

<template>
  <div class="content" @keydown.esc="handleCancel">
    <div class="field-list">
      <div class="field-list__wrapper">
        <NativeListbox
          v-model="selectedField"
          ref="listRef"
          :items="fields"
          @item:selected="handleFieldSelected"
        >
          <template #item="slotProps">
            {{slotProps.order + ". " + slotProps.name}}
          </template>
        </NativeListbox>
      </div>
      <div class="field-list__controls">
        <button @click="handleAddField">Добавить</button>
        <button
          @click="handleRenameField"
          :disabled="selectedField?.immutable"
        >
          Переименовать
        </button>
        <button
          @click="handleDeleteField"
          :disabled="selectedField?.immutable"
        >
          Удалить
        </button>
        <button
          @click="handleMoveUpField"
          :disabled="(selectedField?.order ?? 0) <= 1"
        >
          Выше
        </button>
        <button
          @click="handleMoveDownField"
          :disabled="(selectedField?.order ?? fields.length) >= fields.length"
        >
          Ниже
        </button>
      </div>
    </div>
    <form class="field">
      <div>Описание</div>
      <div>
        <input
          type="text"
          placeholder="Текст, отображаемый когда поле пустое"
          v-model="fieldDescription"
          @blur="handleFieldDescriptionChanged"
        />
      </div>
      <div>Настройки</div>
      <div>
        <input
          type="radio"
          id="sortField"
          name="sortField"
          :checked="selectedField?.is_default"
          @change="handleDefaultFieldChanged"
        />
        <label for="sortField">Сортировать по этому полю</label>
      </div>
    </form>
    <div class="controls">
      <span class="controls__message" v-show="disabled">
        *Добавьте хотя бы одно поле
      </span>
      <button
        :class="{ shake: disabled }"
        :disabled="loading"
        @click="handleConfirm"
      >
        <Loader v-show="loading" />
        <span :class="{ hidden: loading }">Сохранить</span>
      </button>
      <button @click="handleCancel">Отменить</button>
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

input[type="text"] {
  padding: 0.3em 0.6em;
}

button {
  padding: 0.3em 1.2em;
}

.content {
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
  gap: 8px;
  @include user-select-none;
}

.field-list {
  flex: 1;
  display: flex;
  flex-direction: row;
  gap: 8px;
}

.field-list__wrapper {
  width: 100%;
  height: 100%;
  padding: 4px 0;
  border-radius: 4px;
  box-sizing: border-box;
  background-color: #272727;
}

.listbox {
  height: 100%;
}

.field-list__controls {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.field {
  display: grid;
  grid-template-columns: minmax(min-content, 150px) auto;
  gap: 8px;
  text-align: left;
  @include user-select-none;

  div {
    display: flex;
    align-items: center;
  }

  input[type="text"] {
    width: 100%;
  }
}

.controls {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
}

.controls__message {
  margin-right: 2px;
  color: red;
}
</style>
