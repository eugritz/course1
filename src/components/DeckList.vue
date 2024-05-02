<script setup lang="ts">
import { computed, useSlots } from 'vue';
import Badge from './Badge.vue';

const slots = useSlots();
const deckCount = computed(() => slots.default ? slots.default().length : 0);
</script>

<template>
  <div class="deck-list">
    <div class="deck-list__title">
      <h1>Колоды</h1>
      <Badge class="deck-list__title__badge">{{ deckCount }}</Badge>
    </div>
    <ul class="deck-list__list">
      <slot></slot>
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

.deck-list__title__badge {
  min-width: 1em;
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
