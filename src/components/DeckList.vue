<script setup lang="ts">
import { computed, useSlots } from 'vue';
import { invoke } from "@tauri-apps/api";
import { Event, listen } from "@tauri-apps/api/event";

import { useTauriEvent } from 'utils/tauriEvent';

import Badge from './Badge.vue';
import LinkButton from './LinkButton.vue';

const slots = useSlots();
const deckCount = computed(() => slots.default ? slots.default().length : 0);

function openNewDeckDialog() {
  invoke("open_new_deck_dialog");
}

function handleDialogResult(event: Event<unknown>) {
  if (event.windowLabel !== "NewDeckDialog") {
    return;
  }

  // update
}

useTauriEvent("dialog_result", handleDialogResult);
</script>

<template>
  <div class="deck-list">
    <div class="deck-list__title">
      <h1>Колоды</h1>
      <Badge>{{ deckCount }}</Badge>
    </div>
    <ul class="deck-list__list">
      <slot>
        <LinkButton @click="openNewDeckDialog">
          Создать новую колоду
        </LinkButton>
      </slot>
    </ul>
  </div>
</template>

<style scoped lang="scss">
@import "../styles/theme";
@import "../styles/mixins";

.deck-list {
  display: flex;
  flex-direction: column;
  padding: 20px 25px;
  border-radius: 10px;
  box-shadow: 0px 5px 50px rgba(0, 0, 0, 0.1);
  background-color: #ffffff;
}

.deck-list__title {
  display: flex;
  align-items: center;
  
  h1 {
    @include user-select-none;
  }
}

.deck-list__list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  text-align: left;
}

@if $theme == dark {
  .deck-list {
    background-color: #1b1b1b;
    box-shadow: 0px 5px 50px rgba(255, 255, 255, 0.02);
  }
}
</style>
