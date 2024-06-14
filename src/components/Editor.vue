<script lang="ts">
export interface EditorExposed {
  clear(): void;
  getValues(): string[];
  getTags(): string[];
  reset(): void;
}
</script>

<script setup lang="ts">
import { Ref, nextTick, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';

import { EntryFieldValueExtra } from 'entities/EntryFieldValue';
import { EntryKindField } from 'entities/EntryKindField';

import { entryFieldValueStore } from 'stores/entryFieldValueStore';
import { entryKindFieldStore } from 'stores/entryKindFieldStore';
import { entryTagStore } from 'stores/entryTagStore';

import { useTauriEvent } from 'utils/tauriEvent';
import dataEvents from 'constants/dataEvents';
import uiEvents from 'constants/uiEvents';

import EditorSection, { EditorSectionExposed } from './EditorSection.vue';

const props = defineProps<{
  entryId?: number,
  entryKindId?: number,
}>();

const fields = ref<EntryKindField[] | EntryFieldValueExtra[]>([]);
const values = ref<Ref<(EditorSectionExposed | null)[]>[]>([]);
const tagsRef = ref<EditorSectionExposed | null>(null);

useTauriEvent(dataEvents.update.entryKindField, load);
useTauriEvent(uiEvents.window_open, load);

onMounted(load);
watch([() => props.entryId, () => props.entryKindId], load);

function isEntry(_field: EntryKindField | EntryFieldValueExtra):
  _field is EntryFieldValueExtra
{
  return props.entryId !== undefined && props.entryKindId === undefined;
}

function reset() {
  fields.value = [];
  values.value = [];

  if (tagsRef.value)
    tagsRef.value.reset();
}

function load() {
  if (props.entryId !== undefined) {
    entryFieldValueStore.get_fields(props.entryId).then((fields_) => {
      fields_.forEach(() => values.value.push(ref([])));
      fields.value = fields_;

      entryTagStore.get_tags(props.entryId!).then((tags_) => {
        tagsRef.value?.setTags(tags_);
      });

      nextTick(() => {
        for (let i = 0; i < values.value.length; i++) {
          const elem = values.value[i].value[0];
          if (elem && elem.inputRef) {
            const field = fields.value[i];
            if (isEntry(field)) {
              elem.inputRef.value = field.value;
            }
          }
        }
      });
    });
  } else if (props.entryKindId !== undefined) {
    values.value = [];
    entryKindFieldStore.get_fields(props.entryKindId).then((fields_) => {
      fields_.forEach(() => values.value.push(ref([])));
      fields.value = fields_;
    });
  }
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

function handleFieldChange(
  event: Event,
  field: EntryKindField | EntryFieldValueExtra
) {
  if (!isEntry(field))
    return;

  const target = event.target as HTMLInputElement;
  entryFieldValueStore.update(field.id, target.value).then(() => {
    emit(dataEvents.update.entryFieldValue);
  });
}

function handleTagsChange() {
  if (props.entryId === undefined || !tagsRef.value)
    return

  entryTagStore.set_tags(props.entryId, tagsRef.value.getTags() ?? []).then(() => {
    emit(dataEvents.update.tags);
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
  reset,
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
            type="text"
            :ref="values[idx]"
            :key="field.id"
            :title="field.name"
            :placeholder="field.desc"
            @change="handleFieldChange($event, field)"
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
        @change="handleTagsChange"
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
