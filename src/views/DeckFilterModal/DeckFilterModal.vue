<script setup lang="ts">
import NativeListbox from 'components/NativeListbox.vue';
</script>

<template>
  <div class="content">
    <div class="filter">
      Фильтр:
      <input type="text"></input>
    </div>
    <div class="wrapper">
      <NativeListbox class="list" :items="['hello']" />
    </div>
    <div class="controls">
      <button>Выбрать</button>
      <button>Отменить</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../../styles/mixins";

.content {
  height: calc(100vh - 16px);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 1em;
  @include user-select-none;

  input {
    width: 100%;
    padding: 0.3em 0.6em;
  }
}

.wrapper {
  flex: 1;
}

.list {
  height: 100%;
  padding: 4px 0;
  border-radius: 4px;
  background-color: #272727;
}

.controls {
  display: flex;
  justify-content: flex-end;
  gap: 5px;
}
</style>
