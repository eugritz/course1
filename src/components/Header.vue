<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import HomeButton from "./HomeButton.vue";

function openStats() {
  invoke("open_stats_window");
}
</script>

<template>
  <header class="header">
    <ul class="header__button-list">
      <home-button>Главная</home-button>
      <home-button>Словарь</home-button>
      <home-button @click="openStats">Статистика</home-button>
    </ul>
  </header>
</template>

<style scoped lang="scss">
@import "../styles/theme";

.header {
  display: flex;
  justify-content: center;
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  --header-height: 45px;
}

.header__button-list {
  border-radius: 0% 0% 10px 10px;
  box-shadow: 0 5px 50px rgba(0, 0, 0, 0.1);
}

@if $theme == dark {
  .header__button-list {
    box-shadow: 0 5px 100px rgba(255, 255, 255, 0.05);
  }
}
</style>
