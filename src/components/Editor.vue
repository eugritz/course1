<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';

import { EntryKindField } from 'entities/EntryKindField';
import { entryKindFieldStore } from 'stores/entryKindFieldStore';
import { useTauriEvent } from 'utils/tauriEvent';
import uiEvents from 'constants/uiEvents';

import EditorSection from './EditorSection.vue';

const props = defineProps<{
  entryKindId?: number,
}>();

const fields = ref<EntryKindField[]>([]);

useTauriEvent(uiEvents.window_open, load);

function load() {
  if (props.entryKindId === undefined)
    return;

  entryKindFieldStore.fields(props.entryKindId).then((fields_) => {
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
      <EditorSection v-for="field in fields" :title="field.name" type="text" />
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
