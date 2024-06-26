<script setup lang="ts">
import { computed, ref, shallowRef, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api";
import { Event, emit } from "@tauri-apps/api/event";

import { Deck } from "entities/Deck";
import { deckStore } from "stores/deckStore";
import { useTauriEvent } from "utils/tauriEvent";
import dataEvents from "constants/dataEvents";
import uiEvents from "constants/uiEvents";

import Header from "components/Header.vue";
import DeckList from "components/DeckList.vue";
import DeckListItem from 'components/DeckListItem.vue';
import Popup, { PopupExposed } from "components/Popup.vue";

const decks = computed(() => deckStore.cached_all);
const optionsPopup = ref<PopupExposed | null>(null);
const selectedDeck = shallowRef<Deck | null>(null);

useTauriEvent(dataEvents.update.deck, reset);
useTauriEvent(uiEvents.InputModal.onResult, handleRenameDeckDialogResult);
useTauriEvent(uiEvents.ConfirmationModal.onResult, handleDeleteDeckDialogResult);

watchEffect(() => {
  reset();
});

function reset() {
  deckStore.all();
}

function handleOptionsToggle(event: MouseEvent, deck: Deck) {
  selectedDeck.value = deck;
  optionsPopup.value?.toggle(event);
}

function handleOpenRenameDeckDialog() {
  if (!selectedDeck.value) {
    return;
  }

  
  invoke(uiEvents.InputModal.open, {
    id: "rename",
    title: "Переименовать колоду",
    label: "Новое название колоды",
    value: selectedDeck.value.name,
    placeholder: selectedDeck.value.name,
    buttonText: "Изменить",
    loading: true,
  });

  optionsPopup.value?.close();
}

function handleRenameDeckDialogResult(event: Event<unknown>) {
  const payload = event.payload as {
    id: string,
    input: string,
  };

  if (payload.id !== "rename")
    return;

  const newName = payload.input.trim();

  if (!selectedDeck.value || newName.length === 0) {
    emit(uiEvents.InputModal.onReady);
    return;
  }

  deckStore.rename(selectedDeck.value.id, newName).then(() => {
    emit(dataEvents.update.deck);
  }).finally(() => {
    emit(uiEvents.InputModal.onReady);
  });
}

function handleOpenDeleteDeckDialog() {
  if (!selectedDeck.value) {
    return;
  }

  invoke(uiEvents.ConfirmationModal.open, {
    title: "Удалить колоду",
    message: "Вы уверены, что хотите удалить колоду?",
    loading: true,
  });

  optionsPopup.value?.close();
}

function handleDeleteDeckDialogResult(event: Event<unknown>) {
  const payload = event.payload as {
    button: number,
  };

  if (payload.button !== 1 || !selectedDeck.value) {
    emit(uiEvents.ConfirmationModal.onReady);
    return;
  }

  deckStore.delete(selectedDeck.value.id).then(() => {
    emit(uiEvents.ConfirmationModal.onReady);
    reset();
  });
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
          @options:toggle="(e) => handleOptionsToggle(e, item)"
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
