<script lang="ts">
export interface EditorExposed {
  clear(): void;
  getValues(): string[];
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
const values = ref<Ref<string>[]>([]);

useTauriEvent(dataEvents.update.entryKindField, load);
useTauriEvent(uiEvents.window_open, load);

onMounted(load);
watch(() => props.entryKindId, load);

function load() {
  if (props.entryKindId === undefined)
    return;

  values.value = [];
  entryKindFieldStore.get_fields(props.entryKindId).then((fields_) => {
    fields.value = fields_;
    fields_.forEach(() => values.value.push(ref<string>("")));
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
    values.value[i].value = "";
  }
}

function getValues() {
  let values_ = [];
  for (let i = 0; i < values.value.length; i++) {
    const elem = values.value[i].value;
    values_.push(elem);
  }
  return values_;
}

defineExpose({
  clear,
  getValues,
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
      <EditorSection
        v-for="(field, idx) in fields"
        v-model="values[idx].value"
        :key="field.id"
        :title="field.name"
        :placeholder="field.desc"
        type="text"
      />
    </div>
    <div class="separator" />
    <div class="editor__tags">
      <EditorSection class="editor__section__tags" title="Метки" type="tags" />
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
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1em;
}
</style>
