<script setup lang="ts">
import { computed, useSlots } from 'vue';

defineEmits<{
  (e: "options:toggle", event: MouseEvent): void,
}>();

const slots = useSlots();
const title = computed(() => slots.default ? slots.default()[0].children : "");
</script>

<template>
  <li class="deck-list__item">
    <button
      class="deck-list__item__title"
      :title="`Повторение колоды ${title}`"
    >
      <span class="highlight">
        <slot></slot>
      </span>
    </button>
    <button
      class="deck-list__item__options highlight"
      title="Настройки колоды"
      @click="(e) => $emit('options:toggle', e)"
    >
      <unicon class="icon" name="ellipsis-h"></unicon>
    </button>
  </li>
</template>

<style scoped lang="scss">
@import "../styles/theme";

.deck-list__item {
  margin: 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  list-style: none;
}

.deck-list__item__title {
  width: 60%;
  padding: 2px;
  border: none;
  text-align: left;
  box-shadow: none;
  justify-content: start;

  span {
    padding: 5px 6px;
    border-radius: 10px;
  }

  &:active {
    background-color: unset;
  }
}

.deck-list__item__options {
  padding: 0;
  display: flex;
  text-align: left;
  box-shadow: none;

  > .unicon {
    height: 24px;
  }
}

.highlight {
  background-color: #ebf0f7;

  @if $theme == dark {
    background-color: #21262e;
  }
}
</style>
