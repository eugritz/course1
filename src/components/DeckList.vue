<script setup lang="ts">
import { computed, useSlots } from 'vue';
import { invoke } from "@tauri-apps/api";

import events from 'constants/events';

import Badge from './Badge.vue';
import LinkButton from './LinkButton.vue';

const slots = useSlots();
const deckCount = computed(
  () => slots.default ? (slots.default()[0].children?.length ?? 0) : 0
);

function handleOpenNewDeckDialog() {
  invoke(events.DeckNewModal.open);
}
</script>

<template>
  <div class="deck-list">
    <div class="deck-list__header">
      <div class="deck-list__header__title">
        <h1>Колоды</h1>
        <Badge>{{ deckCount }}</Badge>
      </div>
      <div>
        <button
          class="deck-list__header__new-deck"
          title="Создать новую колоду"
          @click="handleOpenNewDeckDialog"
        >
          <unicon class="icon" name="plus"></unicon>
        </button>
      </div>
    </div>
    <ul class="deck-list__list">
      <slot>
        <LinkButton @click="handleOpenNewDeckDialog">
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

.deck-list__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  
  h1 {
    @include user-select-none;
  }
}

.deck-list__header__title {
  display: flex;
  align-items: center;
}

.deck-list__list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  text-align: left;
}

.deck-list__header__new-deck {
  padding: 0;
  background-color: #ebf0f7;
  box-shadow: none;

  @if $theme == dark {
    background-color: #21262e;
  }

  > .unicon {
    height: 24px;
  }
}

@if $theme == dark {
  .deck-list {
    background-color: #1b1b1b;
    box-shadow: 0px 5px 50px rgba(255, 255, 255, 0.02);
  }
}
</style>
