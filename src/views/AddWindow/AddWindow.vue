<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import events from 'constants/events';

function openDeckFilterModal() {
  invoke(events.DeckFilterModal.open);
}
</script>

<template>
  <div class="content">
    <div class="record">
      <div class="record__kind">
        Вид
        <button>Вид</button>
      </div>
      <div class="record__deck">
        Колода
        <button @click="openDeckFilterModal">Колода</button>
      </div>
    </div>
    <div class="wrapper">
    </div>
    <div class="controls">
      <button>Добавить</button>
      <button>Закрыть</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/mixins";

.content {
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
}

.record {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;
  @include user-select-none;
}

.record__kind {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;

  button {
    width: 100%;
    padding: 0.3em 0.6em;
  }
}

.record__deck {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;

  button {
    width: 100%;
    padding: 0.3em 0.6em;
  }
}

.wrapper {
  flex: 1;
}

.controls {
  display: flex;
  justify-content: flex-end;
  gap: 5px;
}
</style>
