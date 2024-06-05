<script lang="ts">
export interface EditorExposed {
  clear(): void;
  getValues(): string[];
  getTags(): string[];
}
</script>

<script setup lang="ts">
import { Ref, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';

import { EntryKindField } from 'entities/EntryKindField';
import { entryKindFieldStore } from 'stores/entryKindFieldStore';
import { useTauriEvent } from 'utils/tauriEvent';
import dataEvents from 'constants/dataEvents';
import uiEvents from 'constants/uiEvents';

import EditorSection, { EditorSectionExposed } from './EditorSection.vue';

const props = defineProps<{
  entryKindId?: number,
}>();

const fields = ref<EntryKindField[]>([]);
const values = ref<Ref<(EditorSectionExposed | null)[]>[]>([]);
const tagsRef = ref<EditorSectionExposed | null>(null);

useTauriEvent(dataEvents.update.entryKindField, load);
useTauriEvent(uiEvents.window_open, load);

onMounted(load);
watch(() => props.entryKindId, load);

function load() {
  if (props.entryKindId === undefined)
    return;

  values.value = [];
  entryKindFieldStore.get_fields(props.entryKindId).then((fields_) => {
    fields_.forEach(() => values.value.push(ref([])));
    fields.value = fields_;
  });
}

function handleOpenEntryKindFieldListWindow() {
  if (props.entryKindId === undefined)
    return;

  emit(uiEvents.EntryKindFieldListWindow.setData, {
    entryKindId: props.entryKindId,
  }).then(() => {
    invoke(uiEvents.EntryKindFieldListWindow.open);
  });
}

function clear() {
  for (let i = 0; i < values.value.length; i++) {
    const elem = values.value[i].value[0];
    elem?.reset();
  }

  if (tagsRef.value)
    tagsRef.value.reset();
}

function getValues() {
  let values_ = [];
  for (let i = 0; i < values.value.length; i++) {
    const elem = values.value[i].value[0];
    if (elem && elem.inputRef)
      values_.push(elem.inputRef.value);
  }
  return values_;
}

function getTags() {
  if (!tagsRef.value)
    return [];
  return tagsRef.value.getTags() ?? [];
}

defineExpose({
  clear,
  getValues,
  getTags,
});
</script>

<template>
  <div class="editor">
    <div class="editor__controls">
      <div>
        <button
          title="Изменить поля вида записи"
          @click="handleOpenEntryKindFieldListWindow"
        >
          Поля...
        </button>
        <button title="Изменить карты вида записи">Карты...</button>
      </div>
      <div>
        <button title="Настройки">
          <unicon class="icon" name="custom-settings" width="18" height="18" />
        </button>
      </div>
      <div>
        <button title="Жирный текст">
          <unicon class="icon" name="bold" width="20" height="20" />
        </button>
        <button title="Курсивный текст">
          <unicon class="icon" name="italic" width="20" height="20" />
        </button>
        <button title="Подчеркнутый текст">
          <unicon class="icon" name="underline" width="20" height="20" />
        </button>
      </div>
      <div>
        <button title="Надстрочный текст">
          <unicon class="icon" name="custom-superscript" width="20" height="20" />
        </button>
        <button title="Подстрочный текст">
          <unicon class="icon" name="custom-subscript" width="20" height="20" />
        </button>
      </div>
      <div> <button title="Прикрепить изображение/аудио/видео">
          <unicon class="icon" name="paperclip" width="17" height="17" />
        </button>
        <button title="Записать голос">
          <unicon class="icon" name="microphone" width="18" height="18" />
        </button>
        <button title="Формулы">
          <unicon class="icon" name="custom-function" width="17" height="17" />
        </button>
      </div>
    </div>
    <div class="separator" />
    <div class="editor__fields">
      <div class="editor__fields__horizontal__wrapper">
        <div class="editor__fields__vertical__wrapper">
          <EditorSection
            v-for="(field, idx) in fields"
            :ref="values[idx]"
            :key="field.id"
            :title="field.name"
            :placeholder="field.desc"
            type="text"
          />
        </div>
      </div>
    </div>
    <div class="separator" />
    <div class="editor__tags">
      <EditorSection
        ref="tagsRef"
        class="editor__section__tags"
        title="Метки"
        type="tags"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
button {
  padding: 0.3em 0.6em;
}

.unicon {
  height: 19px;
}

.editor {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.editor__controls {
  display: flex;
  justify-content: flex-start;
  gap: 4px;

  div {
    display: flex;

    button:first-child {
      border-top-left-radius: 8px;
      border-bottom-left-radius: 8px;
    }

    button {
      min-width: 41px;
      max-height: 31px;
      border-radius: 0;
    }

    button:focus {
      position: relative;
      z-index: 1000;
    }

    button:last-child {
      border-top-right-radius: 8px;
      border-bottom-right-radius: 8px;
    }
  }
}

.separator {
  width: 100%;
  height: 2px;
  box-shadow: 0 2px 0px rgba(0, 0, 0, 0.05);
  background-color: #2b2b2b;
}

.editor__fields {
  position: relative;
  height: 100%;
}

.editor__fields__horizontal__wrapper {
  position: absolute;
  width: calc(100% - 10px);
  height: 100%;
  padding-left: 8px;
  overflow-y: scroll;
}

.editor__fields__vertical__wrapper {
  position: absolute;
  width: calc(100% - 14px);
  padding-bottom: 4px;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1em;
}
</style>
