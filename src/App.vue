<script setup lang="ts">
import { computed, ref, watchEffect } from "vue";
import { Event } from "@tauri-apps/api/event";

import { deckStore } from "stores/deckStore";
import { useTauriEvent } from "utils/tauriEvent";

import Header from "components/Header.vue";
import DeckList from "components/DeckList.vue";
import DeckListItem from 'components/DeckListItem.vue';
import Popup, { PopupExposed } from "components/Popup.vue";

const decks = computed(() => deckStore.cached);
const optionsPopup = ref<PopupExposed | null>(null);

function reset() {
  deckStore.all();
}

useTauriEvent("dialog_result", handleDialogResult);

watchEffect(() => {
  reset();
});

function handleDialogResult(event: Event<unknown>) {
  if (event.windowLabel !== "NewDeckDialog") {
    return;
  }

  reset();
}

function handleOptionsToggle(event: MouseEvent) {
  optionsPopup.value?.toggle(event);
}

function handleOpenRenameDeckDialog() {
  // TODO
  optionsPopup.value?.close();
}

function handleOpenDeleteDeckDialog() {
  // TODO
  optionsPopup.value?.close();
}
</script>

<template>
  <Header />
  <div class="content--center">
    <div class="content--focus">
      <DeckList class="full-width">
        <DeckListItem
          v-for="item in decks"
          :key="item.id"
          @options:toggle="handleOptionsToggle"
        >
          {{ item.name }}
        </DeckListItem>
      </DeckList>
      <Popup class="options" ref="optionsPopup">
        <button class="options__button" @click="handleOpenRenameDeckDialog">
          Переименовать
        </button>
        <button
          class="options__button danger"
          @click="handleOpenDeleteDeckDialog"
        >
          Удалить
        </button>
      </Popup>
    </div>
  </div>
</template>

<style scoped lang="scss">
.full-width {
  width: 100%;
}
</style>

<style lang="scss">
@import "styles/theme";

.options {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.options__button {
  min-width: 100px;
  padding: 4px 10px;
  padding-right: 40px;
  border-width: 0;
  justify-content: start;
  box-shadow: none;
}

.options__button:hover {
  background-color: #ebf0f7;
}

.danger {
  color: #d20000;
}

.danger:hover {
  background-color: #f7ebeb;
}

@if $theme == dark {
  .options__button:hover {
    background-color: #21262e;
  }

  .danger:hover {
    background-color: #2e2121;
  }
}
</style>
